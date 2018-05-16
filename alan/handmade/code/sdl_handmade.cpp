#include <SDL.h>

bool HandleEvent(SDL_Event *Event) {
    bool ShouldQuit = false;
    return(ShouldQuit);
}

int main(int argc, char *argv[]) {
	if (SDL_Init(SDL_INIT_VIDEO) != 0); //TODO: SDL_Init didn't work!
	// SDL_ShowSimpleMessageBox(SDL_MESSAGEBOX_INFORMATION, "Handmade Hero", "This is Handmade Hero", 0);
	//
	//
	SDL_Window* Window;

	Window = SDL_CreateWindow("Handmade Hero",
                          SDL_WINDOWPOS_UNDEFINED,
                          SDL_WINDOWPOS_UNDEFINED,
                          640,
                          480,
                          SDL_WINDOW_RESIZABLE);
	return 0;
}
