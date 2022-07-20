from textual.app import App
from textual.reactive import Reactive
from textual.widgets import Footer, Header

from pythonLib.helpList import helpList
from pythonLib.cmdLine import cmdLine
from pythonLib.interface import interface
from pythonLib.memoryApps import memoryApps

class screen(App):
    show_help = Reactive(False)
    
    async def on_load(self):
        await self.bind("ctrl+a", "toggle_help", "Ajuda  ")
        await self.bind("ctrl+c", "quit", "Saída  ")
    
    def watch_show_help(self, show_help: bool):
        self.helpBar.animate("layout_offset_x", 0 if show_help else 52)
        
    def action_toggle_help(self):
        self.show_help = not self.show_help
    
    async def on_mount(self):
        
        header = Header(tall=False) # Cria o cabecalho
        await self.view.dock(header) # Adiciona o cabecalho no topo
                
        footer = Footer()
        await self.view.dock(footer, edge="bottom", size=1) # Adiciona o rodape
        
        self.helpBar = helpList() # Cria uma barra
        await self.view.dock(self.helpBar, edge="right", size=52, z=1)
        
        self.helpBar.layout_offset_x = 52

        homeGrid = await self.view.dock_grid()
        
        homeGrid.add_row("row1", fraction= 3)
        homeGrid.add_row("row2", fraction= 1, max_size= 13, min_size= 3)
        homeGrid.add_column("col")
        homeGrid.place(interface(), cmdLine())
        
        memoryApps().addLoader()
