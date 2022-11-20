pub const INTEROP_REQUIRE: &str = "interopRequire";
pub const EXPORT_STAR: &str = "exportStar";

// port from webpack RuntimeGlobals

/**
 * the internal require function
 */
pub const REQUIRE: &str = "__webpack_require__";

/**
 * the module cache
 */
pub const MODULE_CACHE: &str = "__webpack_require__.c";

/**
 * the chunk ensure function
 */
pub const ENSURE_CHUNK: &str = "__webpack_require__.e";

/**
 * an object with handlers to ensure a chunk
 */
pub const ENSURE_CHUNK_HANDLERS: &str = "__webpack_require__.f";

/**
 * the bundle public path
 */
pub const PUBLIC_PATH: &str = "__webpack_require__.p";

/**
 * the filename of the script part of the chunk
 */
pub const GET_CHUNK_SCRIPT_FILENAME: &str = "__webpack_require__.u";

/**
 * the filename of the css part of the chunk
 */
pub const GET_CHUNK_CSS_FILENAME: &str = "__webpack_require__.k";

/**
 * function to load a script tag.
 * Arguments: (url: string, done: (event) => void), key?: string | number, chunkId?: string | number) => void
 * done function is called when loading has finished or timeout occurred.
 * It will attach to existing script tags with data-webpack == uniqueName + ":" + key or src == url.
 */
pub const LOAD_SCRIPT: &str = "__webpack_require__.l";

/**
 * the shorthand for Object.prototype.hasOwnProperty
 * using of it decreases the compiled bundle size
 */
pub const HAS_OWN_PROPERTY: &str = "__webpack_require__.o";

/**
 * the module functions, with only write access
 */
pub const MODULE_FACTORIES_ADD_ONLY: &str = "__webpack_require__.m (add only)";

/**
 * register deferred code, which will run when certain
 * chunks are loaded.
 * Signature: (chunkIds: Id[], fn: () => any, priority: int >= 0 = 0) => any
 * Returned value will be returned directly when all chunks are already loaded
 * When (priority & 1) it will wait for all other handlers with lower priority to
 * be executed before itself is executed
 */
pub const ON_CHUNKS_LOADED: &str = "__webpack_require__.O";

/**
 * global callback functions for installing chunks
 */
pub const CHUNK_CALLBACK: &str = "webpackChunk";

/**
 * the module functions
 */
pub const MODULE_FACTORIES: &str = "__webpack_require__.m";

/**
 * interceptor for module executions
 */
pub const INTERCEPT_MODULE_EXECUTION: &str = "__webpack_require__.i";