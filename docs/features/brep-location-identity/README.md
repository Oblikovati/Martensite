# Face definitions and placed instances

A face definition can appear at more than one location in an assembly. Removing locations when counting definitions gives a different result from counting placed instances. These two identities must remain distinct (#237).

The owned ASCII archive contains 14 distinct face definitions and 15 placed face instances, with three shells and three solids in either counting mode. Its repeated location records preserve both sharing and placement. The reader already satisfies this contract; no production behavior or numeric baseline changed.

The regression reads the complete archive through the supported reader and checks both counts. Run it with:

```sh
cargo test -p martensite-tkbrep --test toolkit repeated_location_identity
```
