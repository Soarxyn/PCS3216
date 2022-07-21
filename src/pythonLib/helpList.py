from rich.align import Align
from rich.style import Style
from rich.panel import Panel
from rich.console import RenderableType
from rich.tree import Tree

from textual.widget import Widget

class helpList(Widget):
    
    helpContents = [
        ["[b]ASSEMBLE [i]arquivo[/i] \[-o saida][/]", "Monta [i]arquivo[i/]"],
        ["[b]LINK [i]arquivos[/i] \[-o saida][/]", "Liga [i]arquivo[/i]"],
        ["[b]LOAD [i]arquivo[/]", "Carrega [i]arquivo[/i] na memória"],
        ["[b]UNLOAD [i]arquivo[/]", "Descarrega [i]arquivo[/i] da memória"],
        ["[b]PEEK [i]arquivo[/]", "Abre uma prévia do [i]arquivo[/i]"],
        ["[b]DELETE [i]arquivo[/]", "Apaga [i]arquivo[/i] da pasta [b]src[/]"],
        ["[b]CLEAR[/]", "Limpa a saída do terminal"],
        ["[b]HOME[/]", "Volta para tela inicial"],
        # ["[b]RUN [i]arquivo[/]", "Roda [i]arquivo[/i] de uma vez"],
        ["[b]SIMULATE [i]arquivo[/]", "Simula [i]arquivo[/i] passo a passo"],
        ["[b]SEE[/]", "Mostra a memória escolhida na simulação"],
        ["[b]STEP[/]", "Avança um passo na simulação"],
    ]
    
    helpBar = Tree("Comandos", guide_style= "bold")
        
    for i in range(len(helpContents)):
        helpBar.add(helpContents[i][0]).add(helpContents[i][1])
    
    def render(self) -> RenderableType:
        return Panel(Align(self.helpBar),
                     title= "Comandos",
                     border_style= Style(color= "bright_magenta"))
