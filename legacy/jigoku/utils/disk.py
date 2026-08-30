import os
import re
CLEANR = re.compile('<.*?>') 

def get_size(string) -> float:
    """Get size of a file
    Parameters
    ----------
    string : str
        Path to file
    Returns
    -------
    file : int
        Size of the file
    """
    file = round(os.path.getsize(string) / 1024 / 1024, 2)
    return file

def clean_html(raw_html) -> str:
    """Cleaning some shit
    Parameters
    ----------
    raw_html : str
        String to clean
    Returns
    -------
    cleantext : str
        Cleaned string

    """
    cleantext = re.sub(CLEANR, '', raw_html)
    cleantext = cleantext.strip()
    return cleantext
    