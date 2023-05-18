import Discord from "discord.rs"
import { readFileSync } from "node:fs"


let client = new Discord.Client();

console.log(client)

const config = JSON.parse(readFileSync("./config.json", "utf-8"))

client.login(config?.token)