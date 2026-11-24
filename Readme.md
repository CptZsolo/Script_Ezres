# Hasznos linkek

Offline doksi:
file:///C:/Program%20Files%20(x86)/Steam/steamapps/common/LOTUS-Simulator/lotus_ng/doc/en/scripts.html







LOTUS leírás
https://docs.lotus-simulator.dev/en/rust/quickstart/

Hivatalos git repo-k
https://github.com/Oriolus-Software/LOTUS-Scripting/tree/master/lotus-script
https://github.com/Oriolus-Software/lotus-script-gt6n/blob/observer_version/src/lib.rs

Elvileg hivatalos rust crate-ek:
https://crates.io/users/Ekranos

Leírások a meglévő modulokhoz:
*lotus_script*
https://script-rs.lotus-simulator.dev/lotus_script/all

*lotus_extra*
https://docs.rs/lotus-extra/0.2.2/lotus_extra/all.html

*lotus_rt*
https://docs.rs/lotus-rt/0.1.0/lotus_rt/all.html




# Plugin
Action küldési példa:
curl -v http://localhost:3000/input/action --json "{\"action\": \"Throttle\", \"pressed\": false, \"cockpit_index\": 0 }"


Változó teszt:
curl -H "Content-Type: application/json" -X PUT http://localhost:3000/scripts/12884897122/variables -d "{ \"A_CP_iranyvaltoBent\": { \"type\": \"bool\", \"value\": false }}"

curl -H "Content-Type: application/json" -X PUT http://localhost:3000/scripts/12884897122/variables -d "{ \"JOY_Airbrake\": { \"type\": \"f64\", \"value\": 0.1 }}"



