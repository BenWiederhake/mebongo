let wasm;

const cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}
/**
* @param {number} version
* @param {number} max_board_w
* @param {number} max_board_h
* @param {number} max_tile_size
* @param {number} total_tiles
* @returns {number}
*/
export function check_config(version, max_board_w, max_board_h, max_tile_size, total_tiles) {
    const ret = wasm.check_config(version, max_board_w, max_board_h, max_tile_size, total_tiles);
    return ret >>> 0;
}

/**
* @param {number} tiles_encoded
* @param {number} board_encoded
* @param {number} max_steps
* @returns {Result}
*/
export function compute_result(tiles_encoded, board_encoded, max_steps) {
    const ret = wasm.compute_result(tiles_encoded, board_encoded, max_steps);
    return Result.__wrap(ret);
}

/**
*/
export class Result {

    static __wrap(ptr) {
        const obj = Object.create(Result.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_result_free(ptr);
    }
    /**
    * @returns {number}
    */
    get steps_taken() {
        const ret = wasm.__wbg_get_result_steps_taken(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set steps_taken(arg0) {
        wasm.__wbg_set_result_steps_taken(this.ptr, arg0);
    }
    /**
    * @returns {boolean}
    */
    get has_solution() {
        const ret = wasm.__wbg_get_result_has_solution(this.ptr);
        return ret !== 0;
    }
    /**
    * @param {boolean} arg0
    */
    set has_solution(arg0) {
        wasm.__wbg_set_result_has_solution(this.ptr, arg0);
    }
    /**
    * @returns {boolean}
    */
    get has_finished() {
        const ret = wasm.__wbg_get_result_has_finished(this.ptr);
        return ret !== 0;
    }
    /**
    * @param {boolean} arg0
    */
    set has_finished(arg0) {
        wasm.__wbg_set_result_has_finished(this.ptr, arg0);
    }
    /**
    * @returns {bigint}
    */
    get row0() {
        const ret = wasm.__wbg_get_result_row0(this.ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
    * @param {bigint} arg0
    */
    set row0(arg0) {
        wasm.__wbg_set_result_row0(this.ptr, arg0);
    }
    /**
    * @returns {bigint}
    */
    get row1() {
        const ret = wasm.__wbg_get_result_row1(this.ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
    * @param {bigint} arg0
    */
    set row1(arg0) {
        wasm.__wbg_set_result_row1(this.ptr, arg0);
    }
    /**
    * @returns {bigint}
    */
    get row2() {
        const ret = wasm.__wbg_get_result_row2(this.ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
    * @param {bigint} arg0
    */
    set row2(arg0) {
        wasm.__wbg_set_result_row2(this.ptr, arg0);
    }
    /**
    * @returns {bigint}
    */
    get row3() {
        const ret = wasm.__wbg_get_result_row3(this.ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
    * @param {bigint} arg0
    */
    set row3(arg0) {
        wasm.__wbg_set_result_row3(this.ptr, arg0);
    }
    /**
    * @returns {bigint}
    */
    get row4() {
        const ret = wasm.__wbg_get_result_row4(this.ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
    * @param {bigint} arg0
    */
    set row4(arg0) {
        wasm.__wbg_set_result_row4(this.ptr, arg0);
    }
    /**
    * @returns {bigint}
    */
    get row5() {
        const ret = wasm.__wbg_get_result_row5(this.ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
    * @param {bigint} arg0
    */
    set row5(arg0) {
        wasm.__wbg_set_result_row5(this.ptr, arg0);
    }
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function getImports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    return imports;
}

function initMemory(imports, maybe_memory) {

}

function finalizeInit(instance, module) {
    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;
    cachedUint8Memory0 = null;


    return wasm;
}

function initSync(module) {
    const imports = getImports();

    initMemory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return finalizeInit(instance, module);
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('mebongo_bg.wasm', import.meta.url);
    }
    const imports = getImports();

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    initMemory(imports);

    const { instance, module } = await load(await input, imports);

    return finalizeInit(instance, module);
}

export { initSync }
export default init;
