import os
import sys
import time
from inputimeout import inputimeout, TimeoutOccurred
from jigoku.utils.log import log_time
from jigoku.client.scrape_posts import download_from_multiple_posts
from jigoku.client.scrape_pages import download_from_multiple_pages


try:
    file = inputimeout(
        prompt="[+] Please enter file you want to bulk download (e.g. file.txt): ",
        timeout=30
    )

    download_by = inputimeout(
        prompt="[+] Please choose (1) Multiple posts or (2) Multiple pages: ",
        timeout=30,
    )

    select_type = inputimeout(
        prompt="[+] Select type image results (1) Original size or (2) Smaller size: ",
        timeout=30
    )

except TimeoutOccurred:
    print("Timeout occurred, kindly read the docs: https://github.com/sinkaroid/jigoku#usage")
    os._exit(0)

if not file.endswith(".txt"):
    file = file + ".txt"


def main():
    if download_by == "1" or download_by == "posts" or download_by == "post":
        start = time.time()
        download_from_multiple_posts(file, select_type)
        end = time.time()
        log_time(start, end)

    elif download_by == "2" or download_by == "pages" or download_by == "page":
        start = time.time()
        download_from_multiple_pages(file, select_type)
        end = time.time()
        log_time(start, end)

    else:
        print("Invalid request")
        os._exit(0)


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print("Interrupted")
        try:
            sys.exit(0)
        except SystemExit:
            os._exit(0)
