from cesk.Config import Config

class Rule:
    conf_in: Config
    conf_out: Config

    def __init__(self,conf_in, conf_out):
        self.conf_in = conf_in
        self.conf_out = conf_out

    def __str__(self):
        return f"{self.conf_in} -> {self.conf_out}"