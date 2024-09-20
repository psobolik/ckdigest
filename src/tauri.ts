import {invoke} from "@tauri-apps/api/tauri";

export interface DigestFileParts {
    algorithm: string,
    file: string,
    digest: string,
}

export default class Tauri {
    static calculateDigest = (pathBuf: string, algorithm: string): Promise<string> => {
        return invoke("calculate_digest", {
            pathBuf: pathBuf, algorithm: algorithm
        });
    }
    // We could probably use `open` from '@tauri-apps/api/dialog', but this function
    // knows it is only returning one file, so it's a little more straightforward
    // to use.
    static pickFile = (): Promise<string | null> => {
        return invoke("pick_file");
    }
    static pickDigestFile = (): Promise<string | null> => {
        return invoke("pick_digest_file");
    }
    static parseDigestFile = (digestFile: string): Promise<DigestFileParts> => {
        return invoke("parse_digest_file", {digestFile: digestFile});
    }
    static saveDigestFile = (digest: string, digestedFile: string, hashFunction: string): Promise<string> => {
        return invoke("save_digest_file", {
            digestedFile,
            hashFunction,
            digest
        });
    }
}