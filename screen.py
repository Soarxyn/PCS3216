from rich.layout import Layout

from textual.app import App
from textual.reactive import Reactive
from textual.widgets import Footer, Header

from memoryApps import memoryApps
from folderOpen import folderOpen
from helpList import helpList
from cmdLine import cmdLine
from interface import interface

class screen(App):
    show_help = Reactive(False)
    
    async def on_load(self):
        await self.bind("ctrl+a", "toggle_help", "Help")
        await self.bind("ctrl+r", "refresh_src", "Refresh src")
    
    def watch_show_help(self, show_help: bool):
        self.barra.animate("layout_offset_x", 0 if show_help else 40)
        
    def action_toggle_help(self):
        self.show_help = not self.show_help
    
    def action_refresh_src(self):
        folderOpen().updater()
        
    # O que acontece ao rodar o programa (SETUP)
    async def on_mount(self):
        
        header = Header(tall=False) # Cria o cabecalho
        await self.view.dock(header) # Adiciona o cabecalho no topo
                
        footer = Footer()
        await self.view.dock(footer, edge="bottom") # Adiciona o rodape

        self.barra = helpList() # Cria uma barra
        await self.view.dock(self.barra, edge="right", size=40, z=1)

        homeGrid = await self.view.dock_grid()
        
        homeGrid.add_row("row1", fraction= 3)
        homeGrid.add_row("row2", fraction= 1, max_size= 13, min_size= 3)
        homeGrid.add_column("col")
        homeGrid.place(interface(), cmdLine())
        
        self.barra.layout_offset_x = 40
        
screen.run(log="textual.log", log_verbosity=2, title="PatinhOS :duck:")