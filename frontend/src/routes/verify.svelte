<script>
    import { goto } from "$app/navigation";
import { Routes } from "$lib/api";
    import { client } from "$lib/client";
    import { LangKey as lk, language } from "$lib/lang";


    let params = new URLSearchParams(window.location.search);
    let id = params.get("id");

    if(id) {
        client.confirm_access({
            callback: async function(identifier) {
                await Routes.Auth.VERIFY.send({
                    identifier,
                    id
                });
                goto("/");
            },
            description: {
                hint: language[lk.VERIFY_CONFIRM_HINT]
            }
        });
    } else goto("/");
</script>