#!/usr/bin/env python3
import json
import requests

ouput_path = "../src/picker_struct.rs"
emoji_struct_f_string = open("templates/emoji_struct.rs", "r").read()
rust_module_f_string = open("templates/rust_module.rs", "r").read()

# url for emoji data
emoji_json_url = "https://raw.githubusercontent.com/github/gemoji/master/db/emoji.json"

# the underlying json is structured as
# List<Object>
# Where Object has fields
# {
#   emoji: Char
#   description: String
#   category: String
#   aliases: List<String>
#   tags: List<String>
#   unicode_version: Float -- unused
#   ios_version: Float -- unused
# }


def format_struct(obj):
    return emoji_struct_f_string.format(emoji=obj["emoji"],
                                        tags='["' +
                                        '","'.join(obj["tags"]) + '"]',
                                        skintone_modifier="true" if obj["skintone_modifer"] else "false")


def extract_section_indices(json_list):
    prev_cat = ""
    section_index = 0
    section_tuple = [0, 0, 0, 0, 0, 0, 0, 0, 0]
    for i, obj in enumerate(json_list):
        if obj["category"] != prev_cat:
            prev_cat = obj["category"]
            section_tuple[section_index] = i
            section_index = section_index + 1
    return section_tuple


def output_to_file():
    print("pushing output to file")


def combine_tags(obj):
    obj["tags"].extend(
        list(map(lambda x: x.replace("_", " "), obj["aliases"])))
    obj["tags"].insert(0, obj["description"])
    obj["tags"] = list(dict.fromkeys(obj["tags"]))
    return obj["tags"]


def main():
    raw_json = json.loads(requests.get(emoji_json_url).text)
    unic_v = "unicode_version"

    # i remove all male and female modifiers
    # unfortunately qt has an emoji rendering bug
    # and this crate targets Qt. just remove line ~67 to fix that.
    filtered_json = list(filter(
        lambda obj:
        float(obj[unic_v] if obj[unic_v] else "13.0") < 12.0
        and not ("♀" in obj["emoji"] or "♂" in obj["emoji"]),
        raw_json))

    raw_obj = list(map(
        lambda obj: {"emoji": "\"" + obj["emoji"] + "\"",
                     "tags": combine_tags(obj),
                     "skintone_modifer": (obj.get("skin_tones") != None)}, filtered_json))

    section_tuple = extract_section_indices(filtered_json)

    struct_strings = [format_struct(i) for i in raw_obj]

    struct_strings = ",".join(struct_strings)
    struct_strings = "[\n" + struct_strings + "]"

    sne_index = section_tuple[0]
    pnb_index = section_tuple[1]
    ann_index = section_tuple[2]
    fnd_index = section_tuple[3]
    tnp_index = section_tuple[4]
    a_index = section_tuple[5]
    o_index = section_tuple[6]
    s_index = section_tuple[7]
    f_index = section_tuple[8]

    module_string = rust_module_f_string.format(
        emoji_data=struct_strings, length=len(raw_obj),
        pnb_index=pnb_index,
        sne_index=sne_index,
        ann_index=ann_index,
        fnd_index=fnd_index,
        tnp_index=tnp_index,
        a_index=a_index,
        o_index=o_index,
        s_index=s_index,
        f_index=f_index)

    open(ouput_path, "w").write(module_string)


if __name__ == "__main__":
    main()
