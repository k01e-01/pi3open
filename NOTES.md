# developer notes

this is not documentation!

## in format

```
{
    "Username": <username>,
    "Name": <name>,
    "Code": <code>,
    "Apikey": <apikey>,

    "Intensity": <intensity>,
    "Duration": <duration>,
    "Op": <op>,
}
```

## convert 

```
drop username

customname = name
id = code
token = apikey

intensity = intensity
duration = duration * 1000  // s to ms

type = match op {
    0 => "Shock",
    1 => "Vibrate",
    2 => "Sound",
}
```

## out format

```
OpenShockToken: <token>

{
    "shocks": [
        "id": <id>,
        "type": <type>,
        "intensity": <intensity>,
        "duration": <duration>,
        "exclusive": true,
    ],
    "customName": <customname>,
}
```
