// Importando todos os itens do módulo prelude da biblioteca bracket-lib

use bracket_lib::prelude::*;

// Definindo a estrutura (struct) que guarda o estado do jogo
struct State {}

// Implementando o trait GameState para a estrutura State
impl GameState for State {
    // Método chamado a cada iteração do jogo
    fn tick(&mut self, ctx: &mut BTerm) {
        // Limpa o conteúdo do terminal
        ctx.cls();

        // Imprime o texto "" nas coordenadas (25, 25)
        ctx.print(25, 25, "Fala meu amigo figas!");
    }
}

// Função principal que retorna um BError em caso de erro
fn main() -> BError {
    // Criando um contexto de jogo com configurações para um terminal de 80x50 caracteres
    let context = BTermBuilder::simple80x50()
        // Definindo o título da janela do terminal como "Flappy Dragon"
        .with_title("Lectures of rust")
        // Construindo o contexto do jogo e lidando com possíveis erros
        .build()?;
    
    // Iniciando o loop principal do jogo, passando o contexto e uma instância inicial do estado do jogo
    main_loop(context, State{})
}