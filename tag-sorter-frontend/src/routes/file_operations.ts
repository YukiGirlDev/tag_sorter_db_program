
export function file_name(path: string): string {
    let paths = path.split("/");
    return paths[paths.length - 1];
}
export function file_name_without_ext(path: string) {
    let start = file_name(path)
    let split = start.split(".");
    let ext = split.pop();
    let name = split.join(".");
    return [name, ext]
}
export function file_name_raw(path: string): string {
    let start = file_name(path)
    let split = start.split(".");
    let ext = split.pop();
    let name = split.join(".");
    return name;
}
export function file_ext(name: string): string|undefined {
    return name.split(".").pop()
}