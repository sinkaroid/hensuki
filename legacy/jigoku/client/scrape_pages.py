import requests
import os
import time
import re
from bs4 import BeautifulSoup
from jigoku.utils.log import log_data, get_hostname
from jigoku.utils.disk import get_size, clean_html
from jigoku.constant.base import Jigoku

jgx = Jigoku()


def download_from_multiple_pages(file: str, select_type: str) -> None:
    """Request download from multiple pages which is takes all posts and galleries
    Parameters
    ----------
    file : str
        Path to file
    select_type : str
        Type of image to download
    """
    img_count = 0
    img_already_exist = 0
    tags_count = []
    
    with open(f"{file}", "r") as f:
        for line in f:
            if not line.strip():
                continue
            elif not line.startswith(tuple(jgx.supported)):
                raise Exception(f"Unsupported site: {line}")

            elif line.startswith(jgx.e926):
                raise Exception(line.strip(), "Change this to e621 instead")

            elif jgx.validate_links(line):
                raise Exception(f"Invalid list of pages {line.strip()}, expected one or more pages not posts or galleries")

            try:
                res = requests.get(line.strip(), headers=jgx.with_headers)

                if res.status_code == 200:

                    soup = BeautifulSoup(res.text, "html.parser")
                    title = soup.find("title").text

                    tags_count.append(clean_html(title))
                    list_gallery = []

                    if (
                        line.startswith(jgx.safebooru)
                        or line.startswith(jgx.tbib)
                        or line.startswith(jgx.xbooru)
                        or line.startswith(jgx.hypnohub)
                        or line.startswith(jgx.rule34)
                    ):
                        for link in soup.find_all("span", class_="thumb"):
                            list_gallery.append(
                                f'{get_hostname(res.url)}/{link.a["href"]}'
                            )
                    
                    elif line.startswith(jgx.danbooru):
                        for link in soup.find_all("a", class_="post-preview-link"):
                            list_gallery.append(jgx.danbooru + link["href"])

                    elif line.startswith(jgx.allthefallen):
                        for link in soup.find_all("article", class_="post-preview"):
                            list_gallery.append(jgx.allthefallen + link.a["href"])

                    elif line.startswith(jgx.gelbooru):
                        ## get all <article class="thumbnail-preview"> and get a href
                        for link in soup.find_all("article", class_="thumbnail-preview"):
                            list_gallery.append(link.a["href"])

                    elif line.startswith(jgx.realbooru):
                        ## get all href inside <div class="col thumb"
                        for link in soup.find_all("div", class_="col thumb"):
                            list_gallery.append(link.a["href"])

                    elif (
                            line.startswith(jgx.yandere)
                            or line.startswith(jgx.lolibooru)
                            or line.startswith(jgx.konachan)
                            or line.startswith(jgx.konachanNet)
                        ):
                        ## get all <span class="plid">
                        for link in soup.find_all("span", class_="plid"):
                            list_gallery.append(Jigoku.proper_yandere_link(Jigoku.change_protocol(link.text)))

                    elif line.startswith(jgx.e621) or line.startswith(jgx.e926):
                        ## inside <article, get all data-file-url="
                        for link in soup.find_all("article"):
                            list_gallery.append(link["data-file-url"])

                    elif line.startswith(jgx.paheal):
                        ## inside <section id='image-list'> get all second href
                        for link in soup.find("section", id="image-list").find_all("a"):
                            if link["href"].endswith(jgx.expected_format):
                                list_gallery.append(link["href"])

                    ##print(list_gallery)
                    ##print(len(list_gallery))

                    for galeri in list_gallery:
                        current_task = list_gallery.index(galeri) + 1

                        while True:
                            try:
                                if line.startswith(jgx.e621) or line.startswith(jgx.paheal):
                                    image_original = galeri
                                    image_small = galeri
                                
                                else:
                                    res = requests.get(galeri, headers=jgx.with_headers)
                                    soup = BeautifulSoup(res.text, "html.parser")

                                    if (
                                        galeri.startswith(jgx.safebooru)
                                        or galeri.startswith(jgx.tbib)
                                        or galeri.startswith(jgx.xbooru)
                                        or galeri.startswith(jgx.hypnohub)
                                        or galeri.startswith(jgx.rule34)
                                    ):
                                        if (line.startswith(jgx.hypnohub) or line.startswith(jgx.rule34)):
                                            first = str(soup.find("div", class_="link-list").find_all("li"))
                                            struktur_kontol = first.replace(" Original image", "Original image") ## anjing

                                            get_image_original = re.search(r'<a(.+?)>\nOriginal image', struktur_kontol).group(1)
                                            image_original = Jigoku.get_href_value(get_image_original)    
                                            image_small = image_original
                                        else:
                                            first = str(soup.find("div", class_="sidebar").find_all("li"))
                                            get_image_original = re.search(r'<li>(.+?)>Original image</a>', first).group(1)
                                            image_original = Jigoku.get_href_value(get_image_original)     
                                            image_small = image_original
  
                                    elif galeri.startswith(jgx.danbooru) or galeri.startswith(jgx.allthefallen):
                                        first_if_fail = soup.find("section", id="post-options").find("li", id="post-option-download")
                                        image_original = Jigoku.get_href_value(str(first_if_fail.find("a"))).split("?")[0]
                                        image_small = image_original
                                    
                                    elif galeri.startswith(jgx.gelbooru):
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

                                    elif galeri.startswith(jgx.realbooru):
                                        first = soup.find("div", style="text-align: right; font-size: .7em;").find("a")
                                        image_original = first["href"]
                                        image_small = image_original   

                                    elif (
                                        galeri.startswith(jgx.yandere)
                                        or galeri.startswith(jgx.lolibooru)
                                        or galeri.startswith(jgx.konachan)
                                        or galeri.startswith(jgx.konachanNet)
                                    ):
                                        first = str(soup.find("div", class_="sidebar").find_all("li"))
                                        get_image_original = re.search(r'<li><a class="original(.+?)>View larger version', first).group(1)
                                        image_original = Jigoku.get_href_value(get_image_original)
                                        get_image_small = soup.find("div", class_="content").find("img", alt=True)
                                        image_small = get_image_small["src"]
                                    

                                if (
                                    select_type == "1"
                                    or select_type == "o"
                                    or select_type == ""
                                    or select_type == "original"
                                    or select_type == "ori"
                                ):
                                    final_img = image_original
                                    final_name = Jigoku.extract_img_name(final_img)
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

                                try:
                                    ##print(final_img)   
                                        
                                    if not os.path.exists(final_name):
                                        img_count += 1
                                        image = requests.get(final_img, stream=True, headers=jgx.with_headers)
                                        with open(final_name, "wb") as f:
                                            f.write(image.content)
                                        log_data(
                                            f'{current_task} / {len(list_gallery)} | {galeri.split("/")[2]}',
                                            f"{final_name} | Downloaded {get_size(final_name)} MB",
                                        )
                                        break
                                    elif os.path.exists(final_name):
                                        img_already_exist += 1
                                        log_data(
                                            "Skipping... File already exists {}".format(
                                                final_name
                                            ), img_already_exist
                                        )
                                        break

                                except Exception as e:
                                    print(f"Error: {galeri} failed to download: {e} | {res.status_code}")
                                    

                            except Exception as e:
                                if 'NoneType' in str(e):
                                    print(f"{galeri} Skipping with no continue because {e} | Check failed.log for more info")
                                    with open("failed.log", "a") as f:
                                        f.write(galeri + "\n")
                                    break
                                else:
                                    print(f"{galeri} Retrying duck because {e} in 3 seconds.. | {res.status_code}")
                                    time.sleep(3)
                                    continue

                    log_data(
                        f"Downloaded {img_count} contents", f"which is comes from: {tags_count} with {len(tags_count)} pages"
                    )

                else:
                    print(f"Error: {res.status_code}")

            except Exception as e:
                print(f"Error: {e}")
                break
