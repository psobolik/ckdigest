import React from "react";
import "./App.css";
import TitleBar from "./TitleBar.tsx";
import Tauri from "./tauri.ts";

const App: React.FunctionComponent = () => {
    const [digest, setDigest] = React.useState<string>("");
    const [digestedFile, setDigestedFile] = React.useState<string>("");
    const [algorithm, setAlgorithm] = React.useState<string>("SHA256");
    const [expectedDigest, setExpectedDigest] = React.useState<string>("");
    const [match, setMatch] = React.useState<string>("");
    const [message, setMessage] = React.useState<string>("");
    const [loading, setLoading] = React.useState(false);

    React.useEffect(() => {
        setMatch("");
        setMessage("");
    }, [expectedDigest])

    React.useEffect(() => {
        if (!digest) return;

        if (loading) setLoading(false)
        if (expectedDigest) compareDigests();
    }, [digest])

    React.useEffect(() => {
        if (!loading) calculateDigest(digestedFile, algorithm).catch(console.error);
    }, [algorithm])

    async function calculateDigest(file: string | null, algorithm: string) {
        setMessage("");
        setMatch("");
        if (!file) {
            setDigest("");
            setDigestedFile("");
            return;
        }
        setLoading(true);
        setDigest("");
        setDigestedFile(file);
        Tauri.calculateDigest(file, algorithm).then(setDigest)
    }

    function compareDigests() {
        setMessage("");
        setMatch("");
        let has_digest = digest.length !== 0;
        let has_expected = expectedDigest.length !== 0;
        if (has_digest && has_expected) {
            if (digest === expectedDigest) {
                setMatch("Digests match");
            } else {
                setMessage("Digests do not match");
            }
        } else {
            if (has_expected && !has_digest) {
                setMessage("Select a file and try again");
            } else if (!has_expected && has_digest) {
                setMessage("Enter the expected digest for the file and try again");
            } else {
                setMessage("Select a file, enter its expected digest and try again");
            }
        }
    }

    async function generateDigest(_e: React.MouseEvent) {
        Tauri.pickFile()
            .then((file: string | null) => {
                if (file) calculateDigest(file, algorithm).then()
            })
    }

    async function openDigestFile(_e: React.MouseEvent) {
        const digest_file = await Tauri.pickDigestFile().catch(console.error);
        if (!digest_file) return;
        Tauri.parseDigestFile(digest_file)
            .then(digestFileParts => {
                setMessage("");
                setMatch("");
                setDigestedFile("");
                setExpectedDigest(digestFileParts.digest);
                calculateDigest(digestFileParts.file, digestFileParts.algorithm);
                setAlgorithm(digestFileParts.algorithm);
            })
            .catch(err => alert(err.message));
    }

    function clear(_e: React.MouseEvent) {
        setDigest("");
        setDigestedFile("");
        setExpectedDigest("");
        setMessage("");
        setMatch("");
    }

    return (<>
        <div className="container">
            <TitleBar/>
            <div id="control-container">
                <p id="digest-file-display" className={digestedFile ? "" : "hidden"}>{digestedFile}</p>
                <p id="digest-display" className={digest ? "" : "hidden"}>{digest}</p>
                <div className={loading ? "please-wait" : "please-wait hidden"} id="please-wait">
                    <div>&nbsp;</div>
                </div>
                <div className="row">
                    <button id="generate-button" onClick={generateDigest}>Select File</button>
                    <button id="open-digest-button" onClick={openDigestFile}>Select Digest File</button>
                </div>
                <div className="row">
                    <label htmlFor="digest-select">Algorithm</label>
                    <select id="digest-select" value={algorithm} onChange={e => setAlgorithm(e.target.value)}>
                        <optgroup label="Obsolete">
                            <option value="MD5">MD5</option>
                            <option value="SHA1">SHA-1</option>
                        </optgroup>
                        <optgroup label="SHA-2">
                            <option value="SHA224">SHA-224</option>
                            <option value="SHA256">SHA-256</option>
                            <option value="SHA384">SHA-384</option>
                            <option value="SHA512">SHA-512</option>
                            <option value="SHA512_224">SHA-512/224</option>
                            <option value="SHA512_256">SHA-512/256</option>
                        </optgroup>
                        <optgroup label="SHA-3">
                            <option value="Sha3_224">SHA3-224</option>
                            <option value="Sha3_256">SHA3-256</option>
                            <option value="Sha3_384">SHA3-384</option>
                            <option value="Sha3_512">SHA3-512</option>
                        </optgroup>
                    </select>
                </div>
                <div className="row">
                    <label htmlFor="compare-input">Expected</label>
                    <input type="text" placeholder="Expected Digest" id="compare-input"
                           value={expectedDigest}
                           onChange={e => setExpectedDigest(e.target.value)}
                    />
                    <button id="compare-button" onClick={compareDigests}>Compare</button>
                </div>
                <p id="message-display" className={match ? "match" : message ? "message" : ""}>{match}{message}</p>
                <div className="row">
                    <button id="clear-button" onClick={clear}>Clear</button>
                </div>
            </div>
        </div>
    </>);
}

export default App;
