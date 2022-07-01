from textual.app import App
from textual.reactive import Reactive
from textual.widgets import Footer, Header

from memoryApp import memoryApps
from folderOpen import folderOpen
from helpList import helpList
from cmdLine import cmdLine

class Interface(App):
    show_help = Reactive(False)
    
    async def on_load(self):
        await self.bind("ctrl+a", "toggle_help", "Toggle Help")    
    
    def watch_show_help(self, show_help: bool):
        self.barra.animate("layout_offset_x", 0 if show_help else 40)
        
    def action_toggle_help(self):
        self.show_help = not self.show_help
        
    # O que acontece ao rodar o programa (SETUP)
    async def on_mount(self):
        
        header = Header(tall=False) # Cria o cabecalho
        await self.view.dock(header) # Adiciona o cabecalho no topo
                
        footer = Footer()
        await self.view.dock(footer, edge="bottom") # Adiciona o rodape

        self.barra = helpList() # Cria uma barra
        await self.view.dock(self.barra, edge="right", size=40, z=1)

        grid = await self.view.dock_grid()
        grid.add_row("row1", size= 21)
        grid.add_row("row2", size= 7)
        grid.add_column("col", repeat=2)
        grid.add_areas(
            cmd="col1-start|col2-end, row2" # 122 char width
        )
        grid.place(folderOpen(), memoryApps(), cmd=cmdLine("Linha de comando"))
        
        self.barra.layout_offset_x = 40
        
Interface.run(log="textual.log", log_verbosity=2, title="PatinhOS :duck:")