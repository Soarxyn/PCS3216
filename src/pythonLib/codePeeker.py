import os

from rich.style import Style
from rich.console import RenderableType
from rich.panel import Panel
from rich.syntax import Syntax
from rich.text import Text
from rich.align import Align

from textual.widget import Widget
from textual.reactive import Reactive

class _codePeeker(Widget):
    _instance = None
    
    path = Reactive("")
    firstLine = Reactive(1)
    lineCount = 1
    height = 1
    lastLine = 2
    blankText = Text("Rode ").append("Peek ", style= Style(bold= True)).append("arquivo ", style= Style(bold= True, italic= True)).append("para\n    ver um arquivo")
    
    def setPath(self, path: str):
        self.path = "./root/" + path
        self.firstLine = 1
        with open(self.path, 'r') as fp:
            for self.lineCount, line in enumerate(fp):
                pass
        self.lineCount += 1 # Started on 0
    
    def render(self) -> RenderableType:
        self.height = int(3*os.get_terminal_size()[1]/4)
        self.height = max(self.height, os.get_terminal_size()[1] - 13)
        self.height = min(self.height, os.get_terminal_size()[1] - 3)
        self.height -= 7
        self.lastLine = self.firstLine + self.height
        if os.path.exists(self.path):
            codeOpen = Syntax.from_path(
                path= self.path,
                line_range= [self.firstLine, self.lastLine],
                line_numbers= True,
                word_wrap= True,
                indent_guides= True,
                theme= "monokai",
            )
        else:
            codeOpen = Align.center(self.blankText, vertical= "middle")
            
        return Panel(codeOpen,
                     title= self.path[7:],
                     border_style= Style(color= "bright_cyan"))
        
def codePeeker(mode: str):
    if _codePeeker._instance is None:
        _codePeeker._instance = [_codePeeker(), _codePeeker()]
    if mode == "Home":
        return _codePeeker._instance[0]
    return _codePeeker._instance[1]