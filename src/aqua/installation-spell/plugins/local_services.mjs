import { existsSync, readFileSync } from 'fs';


export function plugins() {
    return {
        local_services: {
            /*
            data LocalModule:
                name: string
                path: string
                preopened_files: ?[]string
                mapped_dirs: ?[][]string

            data LocalService:
                name: string
                modules: []LocalModule
            */
            get: (name) => {
                const data = readFileSync(`./artifacts/${name}/deploy.json`);
                const config = JSON.parse(data.toString("utf-8"));

                const names = Object.keys(config);
                if (names.length != 1) { throw "deploy.json must contain a single service, was", names.length };
                const service_name = names[0];

                const modules = config[service_name].modules;
                return {
                    name: service_name,
                    modules
                }
            }
        }
    }
}