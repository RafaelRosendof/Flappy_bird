#![warn(clippy::pedantic)]

use bracket_lib::prelude::*;

const SCREEN_WIDTH : i32 = 80; //largura da tela
const SCREEN_HEIGHT : i32 = 50; //altura da tela
const FRAME_DURATION : f32 = 30.0; //Duração dos frames, quanto menor mais rápido

struct Player { //"Classe player" componete x y e a velocidade do player 
  x: i32, // posição horizontal
  y: i32, //posição vertical 
  velocity: f32,// a velocidade vertical 
}

impl Player { // fazendo um impl para criar as propriedades do player  
  fn new(x: i32, y: i32) -> Self { // função para criar um novo jogador com as posições iniciais
    Player {
      x,
      y,
      velocity: 0.0, //velocidade inicial
    }
  }
  //aplica a gravidade e move o jogador 
  fn gravity_and_move(&mut self) {
     // Incrementando a velocidade de caida
     if self.velocity < 1.0 {
      self.velocity += 0.15; //aqui se define a velocidade de caida 
    }

    // aplicamos a velocidade vertical 
    self.y += self.velocity as i32;
    if self.y < 0 { //Zera a velocidade vertical se estiver caindo abaixo do limite
      self.y = 0;
    }

    // Move o jogador horizontalmente a uma velocidade constante
    self.x += 1; //o player se move 1 pixel por vez 
  }

  fn flap(&mut self) {
    self.velocity = -1.5; //velocidade de subida do pato
  }

  fn render(&mut self, ctx: &mut BTerm) { //Essa função vai definir como será o PATO 
    ctx.set( //set as propriedades do pato
      0, // posição no X sendo 0 ou seja no meio do mapa 
      self.y, //No y tbm a mesma coisa 
      WHITE, //Cor 1 e cor 2
      ORANGE1,
      to_cp437('?') //o que vai ficar dentro do quadrado, no caso uma interrogação 
    );
  }
} //fim da definição das prorpriedades do player 

struct Obstacle {
  x: i32, //posição horizontal do obstaculo
  gap_y: i32,// distancia entre os entre os obstaculos
  size: i32 //tamanho do obstaculo 
}

impl Obstacle {
  //aqui vamos criar um novo obstaculo com base na posição e na pontuação do jogador
  fn new(x: i32, score: i32) -> Self { // função new que tem a posição e o score e retorna um inicializador
    let mut random = RandomNumberGenerator::new();
    //let mut random é um gerador de números aleatórios, e vamos gerar esse número para deixar os obstaculos aleatórios
    Obstacle {
      x, // posição X 
      gap_y: random.range(5, 40), //posição vertical aleatória para a abertura
      size: i32::max(2, 20 - score) //calcula o tamanho do obstaculo com base na pontuação 
    }
  }

// Renderiza o obstáculo no contexto de renderização fornecido, considerando a posição do jogador
  fn render(&mut self, ctx: &mut BTerm, player_x : i32) {
    let screen_x = self.x - player_x; //calcula a posição na tela considerando a posição do jogador
    let half_size = self.size / 2; //metade do tamaho do obstáculo 

    // metade superior do obstáculo 
    for y in 0..self.gap_y - half_size { //0.. self.gap_y significa espaço 
      ctx.set(
        screen_x,
        y,
        GREEN3, //cores
        BLACK,
        to_cp437('|'), //Formato do obstaculo, no caso um tronco
      );
    }

    // Desenha a metade inferior do obstaculo
    for y in self.gap_y + half_size..SCREEN_HEIGHT {
      ctx.set(
        screen_x,
        y,
        PINK4,
        BLUEVIOLET,
        to_cp437('|'),
      );
    }
  }

  // Verifica se o jogador colidiu com o obstáculo
  fn hit_obstacle(&self, player: &Player) -> bool { //função que detecta colisão, retorna um boleano 
    let half_size = self.size / 2;
    let does_x_match = player.x == self.x;// (1) Verifica se a posição X do jogador coincide com a do obstáculo
    let player_above_gap = player.y < self.gap_y - half_size;// (2) Verifica se o jogador está acima da abertura
    let player_below_gap = player.y > self.gap_y + half_size; //Verifica se o jogador está a baixo da abertura
    does_x_match && (player_above_gap || player_below_gap)// (3) Retorna verdadeiro se houver colisão 
  }
}

enum GameMode { //aqui fica os diferentes artributos que o GameMode pode assumir.
  Menu, //interface menu
  Playing,// Local de jogo
  End, //saída
}

struct State { //Struct do estado do jogo atual, aqui temos alguns componentes que vão precisar serem atualizados durante o jogo 
  player: Player, //Objeto jogador 
  frame_time: f32, //FPS
  obstacle: Obstacle, //Objeto obstáculos 
  mode: GameMode, //Qual o modo de jogo seja END playing ou Menu
  score: i32, //score atual 
}

impl State { // implementando as propriedades da struct State 
  fn new() -> Self {
    State {
      player: Player::new(5, 25),//criando o jogador com as posições iniciais que ele vai ficar 
      frame_time: 0.0,//iniciando o fps 
      obstacle: Obstacle::new(SCREEN_WIDTH, 0), //iniciando os obstaculos e definindo suas propriedades 
      mode: GameMode::Menu, // o estado atual assim que abre a janela 
      score: 0, //iniciando o score com 0
    }
  }

  fn restart(&mut self) { //criando a função start para caso o jogador venha a morrer o jogo se reinicia 
    self.player = Player::new(5, 25); //mesma coisa da função new logo em cima 
    self.frame_time = 0.0;
    self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
    self.mode = GameMode::Playing; //a diferença é que agora o modo atual será de Playing
    self.score = 0;
  }

  fn main_menu(&mut self, ctx: &mut BTerm) { //iniciando a função main do jogo 
    ctx.cls(); //limpando o terminal sempre no começo
    ctx.print_centered(5, "Welcome to Flappy Dragon"); //agora é printado algumas mensagens na tela incluindo a sua posição na janela 
    ctx.print_centered(8, "(P) Play Game"); //por estar como centered não precisa do X somente o Y
    ctx.print_centered(9, "(Q) Quit Game");

    if let Some(key) = ctx.key { //aqui fazemos o tratamento para saber se o jogador quer sair ou somente jogar 
      match key { //verificar qual tecla foi digitada e colocar o que fazer 
        VirtualKeyCode::P => self.restart(),
        VirtualKeyCode::Q => ctx.quitting = true,
        _ => {}
      }
    }
  }

  fn dead(&mut self, ctx: &mut BTerm) { //função para caso a condição que defini como dead aconteça
    ctx.cls(); //limpa o terminal
    ctx.print_centered(5, "You are dead!"); //voce morreu 
    ctx.print_centered(6, &format!("You earned {} points", self.score));
    ctx.print_centered(8, "(P) Play Again"); //mostra os pontos e as opções de jogar novamente ou sair do jogo 
    ctx.print_centered(9, "(Q) Quit Game");

    if let Some(key) = ctx.key {
      match key { //mesma função alí em cima 
        VirtualKeyCode::P => self.restart(),
        VirtualKeyCode::Q => ctx.quitting = true,
        _ => {}
      }
    }
  }

  fn play(&mut self, ctx: &mut BTerm) {
    ctx.cls_bg(NAVY); //define a cor de fundo da tela
    self.frame_time += ctx.frame_time_ms; //adiciona o tempo desde o último quadro ao tempo total decorrido

    //verifica se é hora de atualizar a posição do jogador com base na gravidade
    if self.frame_time > FRAME_DURATION {
      self.frame_time = 0.0;

      self.player.gravity_and_move(); //gravidade aqui 
    }
    //verifica se a telca de espaço foi presisonada para fazer o jogador pular
    if let Some(VirtualKeyCode::Space) = ctx.key {
      self.player.flap();
    }
    //renderiza o jogador na tela 
    self.player.render(ctx);
    //exibe o jogador na tela e a pontuação  
    ctx.print(0, 0, "Press SPACE to flap.");
    ctx.print(0, 1, &format!("Score: {}", self.score)); // (4)

    //renderiza o obstáculo 
    self.obstacle.render(ctx, self.player.x); // (5)
    //verifica se o jogador ultrapassou o obstáculo 
    if self.player.x > self.obstacle.x { // (6)
      self.score += 1; //atualiza a pontuação 
      self.obstacle = Obstacle::new( //criando um novo obstáculo na tela 
          self.player.x + SCREEN_WIDTH, self.score
      );
    }
    //verifica se o jogador ultrapassou o limite vertical ou colidiu com algo 
    if self.player.y > SCREEN_HEIGHT || 
        self.obstacle.hit_obstacle(&self.player)
    {
      self.mode = GameMode::End; //game over caso aconteça isso 
    }
  }
}

impl GameState for State {
  fn tick(&mut self, ctx: &mut BTerm) { //função tick que mostra todas as opções do jogo seja o menu play ou end
    match self.mode {
      GameMode::Menu => self.main_menu(ctx),
      GameMode::End => self.dead(ctx),
      GameMode::Playing => self.play(ctx),
    }
  }
}

//Cria uma instância de 'BTermBuilder' para configurar as propriedades do terminal 
fn main() -> BError {
  let context = BTermBuilder::simple80x50()//tamanho da janela
    .with_title("Flappy Figas")//flappy figas 
    .build()?; //construção e ciração do contexto do terminal. 

  main_loop(context, State::new())
}
