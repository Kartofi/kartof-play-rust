const { ANIME } = require("@consumet/extensions");

const gogoanime = new ANIME.Gogoanime;

async function get_info() {
    await gogoanime.fetchEpisodeSources(process.argv[2]).then(data => {
        if (data && data.sources && data.sources.length > 0){
            data.sources.forEach(element => {
                if (element.quality == "default"){
                    console.log(element.url)
                }
            });
        }else{
            console.log("error")
        }
    })
}
try{
    get_info();
}catch(e){
    console.log("error");
}
