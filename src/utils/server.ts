import { http, invoke } from "@tauri-apps/api"


export const startServer = () => {
    invoke('start_server', { port: 1234 });
}

export const stopServer = async () => {
    const ip = await getIp();
    try {
        await http.fetch(`http://${ip}:1234/down?password=${8910}`);
    } catch (e) {

    }
}

export const getIp = async () => {
    return await invoke('get_current_ip') as String;
}