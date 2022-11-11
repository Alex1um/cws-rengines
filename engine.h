//
// Created by alexium on 17.10.22.
//

#ifndef CWS_RENGINES__ENGINE_H_
#define CWS_RENGINES__ENGINE_H_

enum EventType {
  Keyboard,
  MouseClick,
  MouseWheel,
  Custom,
  ServerSync,
  Message,
  FileInput,
  Command,
  Loop,
  Exit,
};

union EventContainer {
  struct {
    int key;
  } keyboard;
  struct {
    int key;
    int x;
    int y;
  } mouse_click;
  struct {
    int x_dir;
    int y_dir;
    int x;
    int y;
  } mouse_wheel;
  struct {
    int type;
    void *data;
  } custom;
  struct {
    char *data;
  } server_sync;
  struct {
    char *data;
  } server_msg;
  struct {
    char *file_name;
  } file_input;
  struct {
    char *cmd;
  } command;
};

struct Event {
  EventType type;
  EventContainer event;
};

typedef void * EventProvider;
typedef void *const Screen;
typedef void (*Callback)(Event, EventProvider *event_provider);

typedef void *const Scene;
typedef unsigned int ObjectId;
typedef void *const Window;
typedef void *const EventLoop;

extern ObjectId create_object(Scene *scene, int x, int y, int z, int type);

extern Scene create_scene(unsigned long x,
                                unsigned long y,
                                unsigned long z);

extern Window create_window(int res_x, int res_y);

extern void testing();

extern void test_string(char *str);

extern void load_texture(Scene *scene, Window *window, char *path);

extern Screen create_screen();

extern EventLoop create_event_loop(Scene *scene, Window *window, Screen *screen);

extern void throw_event(EventProvider *event_provider, Event event);

extern void start_event_loop(EventLoop loop);

extern void add_event_listener(EventLoop *loop, Event event, Callback callback);

extern void add_keyboard_listener(EventLoop *loop, int key, Callback callback);

extern void remove_object(Scene *scene, ObjectId obj_id);

extern Scene clone_scene(Scene *scene);

extern void change_type(Scene *scene, ObjectId obj_id, int new_type);

extern void output_file(const char *file_name);

extern void add_console_input_provider(EventLoop *loop);

extern void add_file_input_provider(EventLoop *loop);

extern void scene_resize(Scene *scene, int x, int y, int z);

extern void scene_smart_resize(Scene *scene, int x, int y, int z);

extern void change_view(Screen *screen, float scale);

#endif //CWS_RENGINES__ENGINE_H_
