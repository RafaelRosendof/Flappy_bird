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
        GREEN3,
        BLACK,
        to_cp437('|'),
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
      player: Player::new(5, 25),
      frame_time: 0.0,
      obstacle: Obstacle::new(SCREEN_WIDTH, 0),
      mode: GameMode::Menu,
      score: 0,
    }
  }

  fn restart(&mut self) {
    self.player = Player::new(5, 25);
    self.frame_time = 0.0;
    self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
    self.mode = GameMode::Playing;
    self.score = 0;
  }

  fn main_menu(&mut self, ctx: &mut BTerm) {
    ctx.cls();
    ctx.print_centered(5, "Welcome to Flappy Dragon");
    ctx.print_centered(8, "(P) Play Game");
    ctx.print_centered(9, "(Q) Quit Game");

    if let Some(key) = ctx.key {
      match key {
        VirtualKeyCode::P => self.restart(),
        VirtualKeyCode::Q => ctx.quitting = true,
        _ => {}
      }
    }
  }

  fn dead(&mut self, ctx: &mut BTerm) {
    ctx.cls();
    ctx.print_centered(5, "You are dead!");
    ctx.print_centered(6, &format!("You earned {} points", self.score));
    ctx.print_centered(8, "(P) Play Again");
    ctx.print_centered(9, "(Q) Quit Game");

    if let Some(key) = ctx.key {
      match key {
        VirtualKeyCode::P => self.restart(),
        VirtualKeyCode::Q => ctx.quitting = true,
        _ => {}
      }
    }
  }

  fn play(&mut self, ctx: &mut BTerm) {
    ctx.cls_bg(NAVY);
    self.frame_time += ctx.frame_time_ms;
    if self.frame_time > FRAME_DURATION {
      self.frame_time = 0.0;

      self.player.gravity_and_move();
    }
    if let Some(VirtualKeyCode::Space) = ctx.key {
      self.player.flap();
    }
    self.player.render(ctx);
    ctx.print(0, 0, "Press SPACE to flap.");
    ctx.print(0, 1, &format!("Score: {}", self.score)); // (4)

    self.obstacle.render(ctx, self.player.x); // (5)
    if self.player.x > self.obstacle.x { // (6)
      self.score += 1;
      self.obstacle = Obstacle::new(
          self.player.x + SCREEN_WIDTH, self.score
      );
    }
    if self.player.y > SCREEN_HEIGHT || 
        self.obstacle.hit_obstacle(&self.player)
    {
      self.mode = GameMode::End;
    }
  }
}

impl GameState for State {
  fn tick(&mut self, ctx: &mut BTerm) {
    match self.mode {
      GameMode::Menu => self.main_menu(ctx),
      GameMode::End => self.dead(ctx),
      GameMode::Playing => self.play(ctx),
    }
  }
}

fn main() -> BError {
  let context = BTermBuilder::simple80x50()
    .with_title("Flappy Dragon")
    .build()?;

  main_loop(context, State::new())
}
