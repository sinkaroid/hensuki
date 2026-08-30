import re
import pkg_resources
import os

version = pkg_resources.require("jigoku")[0].version


class Jigoku:
        
    @staticmethod
    def extract_img_name(url) -> str:
        """Extract image name from url

        Parameters
        ----------
        url : str
            Image url

        Returns
        -------
        str

        """

        image_name = url.split("/")[-1]
        image_name = image_name.split("?")[0]
        
        if len(image_name) > 200:
            ext = image_name.split(".")[-1]
            image_name = image_name[:200 - len(ext) - 1] + "." + ext

        return image_name


    @staticmethod
    def proper_protocols(url: str) -> str:
        if url.startswith("//"):
            return url.replace("//", "https://", 1)
        else:
            return url

    @staticmethod
    def get_href_value(string) -> str:
        """Get href value from long soup

        Parameters
        ----------
        string : str
            String to get href value

        Returns
        -------
        str

        """
        sampah_1 = re.sub(r"#", "", string)
        sampah_2 = re.sub(r"javascript:;", "", sampah_1)
        ## replace all " " with "%20"
        sampah_3 = re.sub(r" ", "%20", sampah_2)
        urls = re.findall(r'href=[\'"]?([^\'" >]+)', sampah_3)
        data = ", ".join(urls)
        return Jigoku.proper_protocols(data)

    @staticmethod
    def proper_yandere_link(string) -> str:
        """Proper yandere link smh

        Parameters
        ----------
        string : str
            String to proper

        Returns
        -------
        str

        """
        string = re.sub(r".*https://", "https://", string)
        string = re.sub(r".*http://", "http://", string)
        return string

    @staticmethod
    def change_protocol(url) -> str:
        """Change http to https, lolibooru doing this shit

        Parameters
        ----------
        url : str
            String to change

        Returns
        -------
        str

        """
        return url.replace("http://", "https://", 1)

    @staticmethod
    def validate_links(url: str) -> bool:
        ## print(f"Mocking... {url.strip()}")
        if (
            not "/posts/" in url ## danbooru & furry based
            and not "&id=" in url ## gelbooru based
            and not "?id=" in url ## gelbooru based
            and not "/show" in url ## yandere based
            and not "/post/view/" in url ## paheal based
        ):   
            return False
        else:
            return True
        

    def __init__(
        self,
        Gelbooru: str = "https://gelbooru.com",
        Safebooru: str = "https://safebooru.org",
        Danbooru: str = "https://danbooru.donmai.us",
        Rule34: str = "https://rule34.xxx",
        Tbib: str = "https://tbib.org",
        Xbooru: str = "https://xbooru.com",
        Realbooru: str = "https://realbooru.com",
        Yandere: str = "https://yande.re",
        Lolibooru: str = "https://lolibooru.moe",
        Konachan: str = "https://konachan.com",
        KonachanNet: str = "https://konachan.net",
        Hypnohub: str = "https://hypnohub.net",
        E621: str = "https://e621.net",
        E926: str = "https://e926.net",
        Allthefallen: str = "https://booru.allthefallen.moe",
        Paheal: str = "https://rule34.paheal.net",
        BASE_headers: dict = {
            "User-Agent": f"jigoku/v{version} (https://pypi.org/project/jigoku);",
            "From": "hey@sinkaroid.org",
        },
        Current_dir: str = os.getcwd(),
        Version: str = version,
        Expected_format: tuple = (
            ".jpg",
            ".png",
            ".gif",
            ".webm",
            ".mp4",
            ".jpeg",
        ),
    ):

        self.gelbooru = Gelbooru
        self.safebooru = Safebooru
        self.danbooru = Danbooru
        self.rule34 = Rule34
        self.tbib = Tbib
        self.xbooru = Xbooru
        self.realbooru = Realbooru
        self.yandere = Yandere
        self.lolibooru = Lolibooru
        self.konachan = Konachan
        self.konachanNet = KonachanNet
        self.hypnohub = Hypnohub
        self.e621 = E621
        self.e926 = E926
        self.allthefallen = Allthefallen
        self.paheal = Paheal
        self.supported = [
            Gelbooru,
            Safebooru,
            Danbooru,
            Rule34,
            Tbib,
            Xbooru,
            Realbooru,
            Yandere,
            Lolibooru,
            Konachan,
            KonachanNet,
            Hypnohub,
            E621,
            E926,
            Allthefallen,
            Paheal,
        ]
        self.with_headers = BASE_headers
        self.current_dir = Current_dir
        self.version = Version
        self.expected_format = Expected_format
