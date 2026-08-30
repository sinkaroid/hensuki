import requests
import os
import time
import re
from bs4 import BeautifulSoup
from jigoku.utils.log import log_data
from jigoku.utils.disk import get_size
from jigoku.constant.base import Jigoku

jgx = Jigoku()


def download_from_multiple_posts(file: str, select_type: str) -> None:
    """Bulk request download from multiple posts or galleries
    Parameters
    ----------
    file : str
        Path to file
    select_type : str
        Type of image to download

    """
    img_count = 0
    img_already_exist = 0

    with open(f"{file}", "r") as f:
        for line in f:
            if not line.strip():
                continue
            elif not line.startswith(tuple(jgx.supported)):
                raise Exception(f"Unsupported site {line}")

            elif line.startswith(jgx.e926):
                raise Exception(line.strip(), "Change this to e621 instead")

            elif not jgx.validate_links(line):
                raise Exception(f"Invalid links {line.strip()}, expected posts or galleries links, not pages")

            total_lines = sum(1 for line in open(f"{file}"))
            while True:
                try:
                    res = requests.get(line.replace("\n", ""), headers=jgx.with_headers)
                    if res.status_code == 200:
                        soup = BeautifulSoup(res.text, "html.parser")

                        if (
                            line.startswith(jgx.safebooru)
                            or line.startswith(jgx.tbib)
                            or line.startswith(jgx.xbooru)
                            or line.startswith(jgx.hypnohub)
                            or line.startswith(jgx.rule34)
                        ):
                            try:
                                if (line.startswith(jgx.hypnohub) or line.startswith(jgx.rule34)):
                                    first = str(soup.find("div", class_="link-list").find_all("li"))
                                    struktur_kontol = first.replace(" Original image", "Original image") ## anjing

                                    get_image_original = re.search(r'<a(.+?)>\nOriginal image', struktur_kontol).group(1)
                                    image_original = Jigoku.get_href_value(get_image_original)
                                    second = soup.find("div", class_="content").find("img")      
                                    image_small = Jigoku.proper_protocols(second["src"].split("?")[0])
                                else:
                                    first = str(soup.find("div", class_="sidebar").find_all("li"))
                                    get_image_original = re.search(r'<li>(.+?)>Original image</a>', first).group(1)
                                    image_original = Jigoku.get_href_value(get_image_original)
                                    second = soup.find("div", class_="content").find("img")      
                                    image_small = Jigoku.proper_protocols(second["src"].split("?")[0])
                            except:
                                first = soup.find("div", class_="content").find("source")
                                image_original = Jigoku.proper_protocols(first["src"].split("?")[0])
                                image_small = image_original

                        elif line.startswith(jgx.danbooru) or line.startswith(jgx.allthefallen):
                                first_if_fail = soup.find("section", id="post-options").find("li", id="post-option-download")
                                image_original = Jigoku.get_href_value(str(first_if_fail.find("a"))).split("?")[0]
                                image_small = image_original

                        elif line.startswith(jgx.gelbooru):
                            try:
                                first = str(soup.find("section", class_="aside").find_all("li"))
                                get_image_original = re.search(r'<li>(.+?)>Original image</a>', first).group(1)
                                image_original = Jigoku.get_href_value(get_image_original)
                                image_small = soup.find("picture").find("img")["src"]
                            except:
                                first = str(soup.find("section", class_="aside").find_all("li"))
                                get_image_original = re.search(r'<li>(.+?)>Original image</a>', first).group(1)
                                image_original = Jigoku.get_href_value(get_image_original)
                                image_small = image_original

                        elif line.startswith(jgx.realbooru):
                            first = soup.find("div", style="text-align: right; font-size: .7em;").find("a")
                            image_original = first["href"]
                            image_small = image_original

                        elif (
                            line.startswith(jgx.yandere)
                            or line.startswith(jgx.lolibooru)
                            or line.startswith(jgx.konachan)
                            or line.startswith(jgx.konachanNet)
                        ):
                            first = str(soup.find("div", class_="sidebar").find_all("li"))
                            get_image_original = re.search(r'<li><a class="original(.+?)>View larger version', first).group(1)
                            image_original = Jigoku.get_href_value(get_image_original)
                            get_image_small = soup.find("div", class_="content").find("img", alt=True)
                            image_small = get_image_small["src"]

                        elif line.startswith(jgx.e621):
                            try:
                                first = soup.find("div", id="image-download-link").find("a")
                                image_original = first["href"]
                                second = soup.find("img", id="image", class_="fit-window")
                                image_small = second["src"]
                            except:
                                first = soup.find("div", id="image-download-link").find("a")
                                image_original = first["href"]
                                image_small = image_original

                        elif line.startswith(jgx.paheal):
                            try:
                                first = soup.find("section", id="Imagemain").find("img")
                                image_original = first["src"]
                                image_small = image_original
                            except:
                                first = soup.find("section", id="Videomain").find("a")
                                image_original = first["href"]
                                image_small = image_original

                        if (
                            select_type == "1"
                            or select_type == "o"
                            or select_type == ""
                            or select_type == "original"
                            or select_type == "ori"
                        ):
                            final_img = image_original
                            final_name = Jigoku.extract_img_name(image_original)
                        elif (
                            select_type == "2"
                            or select_type == "s"
                            or select_type == "smaller"
                            or select_type == "small"
                        ):
                            final_img = image_small
                            final_name = Jigoku.extract_img_name(image_small)
                        else:
                            print("Invalid type")
                            exit()

                        ##print(final_name, final_img)
                        try:
                            if not os.path.exists(final_name):
                                img_count += 1
                                ##current_progress = f"{img_count} / {len(f.readlines()) + 1}"
                                image = requests.get(final_img, stream=True, headers=jgx.with_headers)
                                with open(final_name, "wb") as f:
                                    f.write(image.content)
                                
                                log_data(
                                    f'{img_count} / {total_lines} | {line.split("/")[2]}',
                                    f"{final_name} | Downloaded {get_size(final_name)} MB",
                                )
                                break

                            elif os.path.exists(final_name):
                                img_already_exist += 1
                                log_data(
                                    "Skipping... File already exists {}".format(
                                        final_name
                                    ),
                                    img_already_exist,
                                )
                                break

                        except:
                            print(f"Error: {final_name, final_img} failed to download")

                    else:
                        print(f"Error: {res.status_code}")
                        break

                except Exception as e:
                    if 'NoneType' in str(e):
                        print(f"{line} Skipping with no continue because {e} | Check failed.log for more info")
                        img_already_exist += 1
                        with open("failed.log", "a") as f:
                            f.write(line + "\n")
                        break
                    else:
                        print(f"{line} Retrying because {e} in 3 seconds.. | {res.status_code}")
                        time.sleep(3)
                        continue
                
        log_data(f"Downloaded {img_count} contents", f"Skipped {img_already_exist} contents")
