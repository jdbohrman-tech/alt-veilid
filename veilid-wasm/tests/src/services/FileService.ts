import * as fs from 'fs';
import * as path from 'path';
import type { Services } from '@wdio/types';
import { fileURLToPath } from 'node:url';
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

declare global {
    namespace WebdriverIO {
        interface Browser {
            /**
             * Write data to a file
             * @param filename The path to the file to write
             * @param data The data to write to the file (string, Buffer, or object)
             * @returns Promise that resolves when the file has been written
             */
            writeFile(filename: string, data: string | Buffer | object): Promise<void>;
        }
    }
}

export class FileService implements Services.ServiceInstance {
    /**
     * Constructor for the FileService
     * @param _options Service options (unused)
     * @param _capabilities WebdriverIO capabilities (unused)
     * @param _config WebdriverIO config (unused)
     */
    constructor(
        private _options: Record<string, any> = {},
        private _capabilities: WebdriverIO.Capabilities = {},
        private _config: WebdriverIO.Config = { capabilities: [{}] }
    ) { }

    /**
     * Before hook that gets executed before test execution begins
     * This is where we add our custom commands to the browser object
     */
    before(
        _capabilities: WebdriverIO.Capabilities,
        _specs: string[],
        browser: WebdriverIO.Browser
    ): void {
        /**
         * Add a writeFile command to the browser object
         * @param filename The path to the file to write
         * @param data The data to write to the file
         * @returns Promise that resolves when the file has been written
         */
        browser.addCommand('writeFile', async function (
            filename: string,
            data: string | Buffer | object
        ): Promise<void> {
            const fullPath = path.resolve(__dirname, '../../', filename);
            // Ensure the directory exists
            const dirname = path.dirname(fullPath);
            if (!fs.existsSync(dirname)) {
                fs.mkdirSync(dirname, { recursive: true });
            }

            // If data is an object, stringify it
            const content = typeof data === 'object' && !(data instanceof Buffer)
                ? JSON.stringify(data, null, 2)
                : data;

            // Write the file
            return new Promise<void>((resolve, reject) => {
                fs.writeFile(filename, content, (err: NodeJS.ErrnoException | null) => {
                    if (err) {
                        reject(err);
                    } else {
                        resolve();
                    }
                });
            });
        });
    }
} 