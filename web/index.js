const koa = require("koa")
const chokidar = require("chokidar")
const path = require("path")
const socketIo = require("socket.io")()


const app = new koa()

app.use(require("koa-static")(__dirname))

app.use(require('koa-static')(path.join(__dirname, "../output")))

chokidar.watch(path.join(__dirname, "../output"), {
    depth: 2
}).on("change", ()=>{
    console.log('change')
    socketIo.emit("change")
})

const server = app.listen(8081)

socketIo.listen(server)