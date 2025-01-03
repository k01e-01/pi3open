# pi3open

a PiShock to OpenShock api translation layer ^w^

the api can be found at [https://pi3open.isso.moe/](https://pi3open.isso.moe/), and the documention can be found at [https://pishock.com/](https://pishock.com/)

## usage

> note: the GetShockerInfo endpoint is currently not implemented, this may cause issues with some clients

### 1. get your OpenShock Api Token and Device Id

on the OpenShock website, find the shocker you'd like to use, and press edit, this should bring up a menu, take note of the `ID` field

now, open Api Tokens, and generate a new one, take note of the token provded to you and do not share it

### 2. configure the PiShock client

- ignore the `Username` field, it is unused
- add anything to the `Name` field, it will be used in the OpenShock shocker logs in the `By User` field
- set the `Code` field to the Device ID you got earlier
- set the `Api Key` field to the Api Token you got earlier

> note: these value names may be different from what the client actually calls them

if your PiShock client has an `Api Endpoint` field, set it to `https://pi3open.isso.moe/` and skip the next step, you are done :D

### 3. redirect PiShock api calls to pi3open

> note: HERE BE DRAGONS! unfortunately this step is quite difficult, and may require significant technical and debugging knowledge, good luck, please feel free to contact me @k01e on Discord if you encounter any issues

> note: if you have a possible solution to this problem, please contact me at @k01e on Discord

while this is possible to do using `/etc/hosts` or a dns proxy such as [Acrylic](https://mayakron.altervista.org/support/acrylic/Home.htm) (Windows) or [dnsmasq](https://dnsmasq.org/doc.html) (Linux & MacOS), there are caveats (notably http only), and it is recomended to find an alternate solution that fits your usecase

i will be presenting the most user friendly option, editing `/etc/hosts`

#### Windows

1. fetch the current ip address of `pi3open.isso.moe` [here](https://mxtoolbox.com/SuperTool.aspx?action=a%3api3open.isso.moe&run=toolpage), as of the time of writing it is `140.238.199.3`
2. press `Win + R`, type `notepad C:\Windows\System32\drivers\etc\hosts`, and hit enter
3. append `<IP HERE> do.pishock.com`, replacing `<IP HERE>` with the ip you fetched in step 1
4. save and close the file
5. open Command Prompt as administator
6. run the following command: `ipconfig /flushdns`

if everything is working correctly, opening `http://do.pishock.com/` in the broswer should bring you back here

> note: my server may be assigned a new ip, in which case you will need to reopen the hosts file and change the ip

#### MacOS and Linux

> note: this documentation is unfinished, please feel free to contribute

## support

if you are encountering issues with pi3open, please feel free to reach out to me on Discord @k01e, i am happy to help :3
