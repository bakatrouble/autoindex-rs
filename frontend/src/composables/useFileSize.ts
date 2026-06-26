const useFileSize = (_size: number) => {
    let size = _size;
    for (const prefix of ['B', 'KiB', 'MiB', 'GiB', 'TiB']) {
        if (size <= 2048) {
            return `${size} ${prefix}`;
        }
        size = Math.floor(size / 1024);
    }
    return `${size} PiB`;
}

export default useFileSize;
