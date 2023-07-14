import pygame

WHITE = (255, 255, 255)

def sim_loop():
    run = True
    clock = pygame.time.Clock()

    while run:
        clock.tick(60)
        pygame.display.update()
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                run = False
    pygame.quit()

if __name__ == "__main__":
    pygame.init()
    WIDTH, HEIGHT = 800, 800
    WIN = pygame.display.set_mode((WIDTH, HEIGHT))
    pygame.display.set_caption("Solar System Simulation")
    sim_loop()
    print("End of Simulation")

class Planet:
    def __init__(self, x, y, x_vel, y_vel, radius, color){
        self.x = x
        self.y = y
        self.radius = radius
        self.color = color
        self.x_vel = x_vel
        self.y_vel = y_vel
        
    }