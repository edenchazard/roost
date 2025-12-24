declare module "colorthief" {
  export default class ColorThief {
    getColor(
      img:
        | Buffer
        | ArrayBuffer
        | Uint8Array
        | Uint8ClampedArray
        | Int8Array
        | Uint16Array
        | Int16Array
        | Uint32Array
        | Int32Array
        | Float32Array
        | Float64Array
        | string,
      quality?: number
    ): Promise<[number, number, number]>;

    getPalette(
      img:
        | Buffer
        | ArrayBuffer
        | Uint8Array
        | Uint8ClampedArray
        | Int8Array
        | Uint16Array
        | Int16Array
        | Uint32Array
        | Int32Array
        | Float32Array
        | Float64Array
        | string,
      colorCount?: number,
      quality?: number
    ): Promise<[number, number, number]>;
  }
}
