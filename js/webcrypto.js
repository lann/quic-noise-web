// AES-GCM

export function aesGcmGenerateKey(bits) {
    return window.crypto.subtle.generateKey(
        { name: "AES-GCM", length: bits },
        false,
        ["encrypt", "decrypt"]
    );
}

export function aesGcmEncrypt(key, iv, data) {
    return window.crypto.subtle.encrypt(
        { name: "AES-GCM", iv},
        key,
        data
    );
}

export function aesGcmDecrypt(key, iv, data) {
    return window.crypto.subtle.decrypt(
        { name: "AES-GCM", iv},
        key,
        data
    );
}
