const rust = import("../pkg/aca");

rust.then(m => m.bonjour())
    .catch(console.error);