# THIS IS CURRENTLY NOT FUNCTIONAL

(because im lazy)

---

# pi3open

a fake pishock api that converts incoming requests and forwards them to openshock

the api can be found at [https://pi3open.isso.moe/](https://pi3open.isso.moe/)

## usage

if the pishock client allows you to change the endpoint yourself, simply set it to pi3open.isso.moe

otherwise, use a dns masking utility like [dnsmasq](https://en.wikipedia.org/wiki/Dnsmasq) for macos and linux, or [Acrylic](https://mayakron.altervista.org/support/acrylic/Home.htm) for windows, and redirect dns requests to `do.pishock.com` to `pi3open.isso.moe`

for the configuration of the pishock client
- the username doesn't matter
- name will be used as the customName on openshock
- code will be used as the device id
- apikey will be used as the apikey

if you have any technical difficulties or issues setting this up, feel free to file an issue or contact me directly at @k01e on discord, or via my email k01e.alm07@gmail.com (which i never check :3)

good luck o7
