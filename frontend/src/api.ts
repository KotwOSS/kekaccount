import CryptoJS from "crypto-js";
import { api_base } from "./config";

export type Method = "GET"|"POST"|"PUT"|"DELETE";

export class Route<A>{
    readonly method: Method;
    readonly body: (args: A)=>any;
    readonly headers: (args: A)=>HeadersInit;
    readonly process: (response: Response)=>any;
    readonly get_url: (args: A)=>string;

    constructor(method: Method, get_url: (args: A)=>string, body: (args: A)=>any, 
    headers: (args: A)=>any, process: (response: Response)=>any) {
        this.method = method;
        this.body = body;
        this.headers = headers;
        this.process = process;
        this.get_url = get_url;
    }

    static obody<A>(method: Method, get_url: (args: A)=>string, body: (args: A)=>any, 
    process: (response: Response)=>any) {
        return new Route(method, get_url, body, ()=>({}), process);
    }

    static oheaders<A>(method: Method, get_url: (args: A)=>string, headers: (args: A)=>any, 
    process: (response: Response)=>any) {
        return new Route(method, get_url, ()=>({}), headers, process);
    }

    async send(args: A) {
        const body = this.body(args);
        const headers = this.headers(args);
        const url = this.get_url(args);

        let result = await fetch(`${api_base}${url}`, {
            method: this.method,
            headers: {
                "Content-Type": "application/json",
                ...headers
            },
            body: body?JSON.stringify(body):undefined
        });
        
        return await this.process(result);
    }
}

export class APIError extends Error {
    readonly status: number;
    readonly json: any;

    constructor(msg: string, status: number, json: any = undefined) {
        super(msg);

        this.status = status;
        this.json = json;

        Object.setPrototypeOf(this, APIError.prototype);
    }

    get_message(): string {
        return this.json?this.json.message:this.message;
    }
}

export namespace Routes {
    export async function ok_processor(r: Response) {
        if(r.headers.get("Content-Type") === "application/json") {
            let json = await r.json();
            if(r.status === 200) return json;
            else throw new APIError("", r.status, json);
        } else throw new APIError(await r.text(), r.status);
    }

    export namespace User {
        export const INFO = Route.oheaders("POST", ()=>"/user/info", 
            (args: {token: string})=>({Authorization: args.token}),
            ok_processor);
        
        export const UPDATE = new Route("POST", ()=>"/user/update",
            (args)=>({...args.changes}),
            (args: {token: string, changes: any})=>({Authorization: args.token}),
            ok_processor);
    }

    export namespace Users {
        export const RETRIEVE = Route.obody("GET", (args)=>`/users/${args.id}`, 
            (_args: {id: string})=>undefined,
            ok_processor);
    }

    export namespace Auth {
        export const VERIFY = Route.obody("POST", ()=>"/auth/verify", 
            (args: {id: string, identifier: any})=>({id: args.id, ...args.identifier}),
            ok_processor);

        export const REGISTER = Route.obody("POST", ()=>"/auth/register", 
            (args: {username: string, email: string, password: string, avatar: string})=>args,
            ok_processor);
    
        export namespace Token {
            export const CREATE = Route.obody("POST", ()=>"/auth/token/create", 
                (args: {name: string, permissions: number, identifier: any})=>({name: args.name, permissions: args.permissions, ...args.identifier}),
                ok_processor);
            
            export const DELETE = Route.obody("POST", ()=>"/auth/token/delete", 
                (args: {id: string, identifier: any})=>({id: args.id, ...args.identifier}),
                ok_processor);

            export const INFO = Route.oheaders("POST", ()=>"/auth/token/info", 
                (args: {token: string})=>({Authorization: args.token}),
                ok_processor);

            export const LIST = Route.oheaders("POST", ()=>"/auth/token/list", 
                (args: {token: string})=>({Authorization: args.token}),
                ok_processor);

            export const TERMINATE = Route.oheaders("POST", ()=>"/auth/token/terminate", 
                (args: {token: string})=>({Authorization: args.token}),
                ok_processor);
        }
    }
    
}

export namespace Regex {
    export const EMAIL = /^\w+[\+\.\w-]*@([\w-]+\.)*\w+[\w-]*\.([a-z]{2,18}|\d+)$/g;
}

export function get_identifier(uoe: string, password: string): any {
    return Regex.EMAIL.test(uoe)?
        {email: uoe, password}:
        {username: uoe, password}
}

export function hash_password(password: string): string {
    return CryptoJS.SHA512(password).toString();
}

export function get_login_redirect(): string {
    let params = new URLSearchParams();
    params.set("r", window.location.pathname + window.location.search);
    return "/login?" + params.toString();
}

export async function is_authorized(): Promise<boolean> {
    let token = localStorage.getItem("token")
    if(token) {
        try {
            let info = await Routes.Auth.Token.INFO.send({token});
            return info.token.perms === -1;
        } catch(e) { 
            return false;
        }
    } else return false;
}