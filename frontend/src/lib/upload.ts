import { upload_file_chunk, upload_chunk, upload_api } from "./config";
import CryptoJS from "crypto-js";

export function none() {}

export type UploadJob = {
	file;
	extension: string;
	on_start: () => void;
	on_progress: (progress: number) => void;
	on_complete: (id: string) => void;
	on_error: (error: string) => void;
	on_finally: () => void;
};

let queue: number[] = [];
let queue_index: number = 0;
let jobs: { [key: number]: UploadJob } = {};

let running: number = 0;
let active: number = 0;

let cancel_callback: () => void;

export function array_buffer_to_word_array(ab) {
	let i8a = new Uint8Array(ab);
	let a = [];
	for (let i = 0; i < i8a.length; i += 4) {
		a.push((i8a[i] << 24) | (i8a[i + 1] << 16) | (i8a[i + 2] << 8) | i8a[i + 3]);
	}
	return CryptoJS.lib.WordArray.create(a, i8a.length);
}

export namespace api {
	export async function make_api_request(path: string, data: ArrayBuffer): Promise<string> {
		return new Promise(function (resolve, reject) {
			let xmlHttp = new XMLHttpRequest();
			xmlHttp.onreadystatechange = function () {
				if (xmlHttp.readyState === 4) {
					(xmlHttp.status === 200 ? resolve : reject)(xmlHttp.response);
				}
			};
			xmlHttp.open("POST", `${upload_api}${path}`, true);
			xmlHttp.send(data);
		});
	}

	export async function create(ext: string) {
		return await make_api_request(`c/${ext}`, null);
	}

	export async function upload(stream: string, hash: string, chunk: ArrayBuffer) {
		return await make_api_request(`u/${stream}/${hash}`, chunk);
	}

	export async function finish(stream: string, hash: string) {
		return await make_api_request(`f/${stream}/${hash}`, null);
	}

	export async function remove(stream: string) {
		return await make_api_request(`r/${stream}`, null);
	}
}

export function cancel(id: number, on_cancel: () => void) {
	if (active === id) cancel_callback = on_cancel;
	else {
		queue = queue.filter(function (i) {
			return i !== id;
		});
		delete jobs[id];
		on_cancel();
	}
}

export function upload(
	file: File,
	extension: string,
	on_start: () => void = none,
	on_progress: (progress: number) => void = none,
	on_complete: (id: string) => void = none,
	on_error: (error: string) => void = none,
	on_finally: () => void = none
): number {
	let id = queue_index++;
	jobs[id] = {
		file,
		extension,
		on_start,
		on_progress,
		on_complete,
		on_error,
		on_finally
	};
	queue.push(id);
	work();
	return id;
}

export async function work() {
	if (running++ === 0) {
		while (queue.length > 0) {
			let id = queue.shift();
			active = id;
			let job = jobs[id];
			delete jobs[id];
			await do_job(job);
			active = null;
		}
		running = 0;
	}
}

export async function do_job(job: UploadJob) {
	job.on_start();

	let file: File = job.file;

	let extension: string = encodeURIComponent(job.extension);

	let hasher = CryptoJS.algo.SHA1.create();

	let stream: string = await api.create(extension);

	let running: boolean = true;

	for (let i = 0; i < file.size && running; i += upload_file_chunk) {
		await new Promise(function (resolve, reject) {
			let slice = file.slice(i, i + upload_file_chunk);

			let reader = new FileReader();
			reader.onload = async function (e) {
				let result = e.target.result as ArrayBuffer;
				for (let f = 0; f < result.byteLength && running; f += upload_chunk) {
					let success = false;

					let chunk = result.slice(f, f + upload_chunk);
					let chunk_hash = CryptoJS.SHA1(array_buffer_to_word_array(chunk)).toString();

					while (!success) {
						if (cancel_callback) {
							running = false;
							break;
						}

						try {
							await api.upload(stream, chunk_hash, chunk);
							success = true;

							job.on_progress((i + f) / file.size);
						} catch (e) {}
					}

					hasher.update(array_buffer_to_word_array(chunk));
				}

				resolve(null);
			};

			reader.readAsArrayBuffer(slice);
		});
	}

	if (running) {
		await api
			.finish(stream, hasher.finalize().toString())
			.then(job.on_complete)
			.catch(job.on_error)
			.finally(job.on_finally);
	} else {
		await api
			.remove(stream)
			.then(cancel_callback)
			.catch(job.on_error)
			.finally(function () {
				cancel_callback = null;
				job.on_finally();
			});
	}
}
