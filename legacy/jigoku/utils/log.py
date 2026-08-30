import logging
import os
logging.basicConfig(format="%(asctime)s - %(message)s", level=logging.INFO)

def log_data(case, note) -> str:
    """Logging data
    Parameters
    ----------
    case : str
        Case of the log
    note : str
        Note of the log
    """
    logging.info(f"{case} {note}")

def log_time(start, end) -> str:
    """Logging time taken

    Parameters
    ----------
    start : float
        Initial start
    end : float
        End date
    """
    logging.info("Task finished took {} min, {} sec".format(int((end - start) / 60), int((end - start) % 60)))
    
def get_hostname(url) -> str:
    """Get hostname from url
    Parameters
    ----------
    url : str
        URL
    Returns
    -------
    hostname : str
        Hostname
    """
    return f'https://{url.split("/")[2]}'
    