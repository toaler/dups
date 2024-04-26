const logger = {
    info: (msg) => console.log(`[INFO] ${new Date().toISOString()}: ${msg}`),
    error: (msg, error) => console.error(`[ERROR] msg="${new Date().toISOString()}: ${msg}" error="${error}"`),
    debug: (msg) => console.debug(`[DEBUG] ${new Date().toISOString()}: ${msg}`),
};

export default logger;