//
// Created by alexium on 17.10.22.
//

#ifndef CWS_RENGINES__ENGINE_H_
#define CWS_RENGINES__ENGINE_H_

typedef void (*Callback)(void);

extern unsigned int create_object(void **scene, int x, int y, int z, int type);

extern void *const create_scene(unsigned long x, unsigned long y, unsigned long z);

extern void *const create_window(int res_x, int res_y);

extern void testing();

extern void test_string(char *str);

extern void load_texture(void **scene, void **window, char *path);

extern void *create_event_loop(void **scene, void *window);

extern void start_event_loop(void *loop);

extern void add_event_listener(void **loop, Callback callback);

extern void remove_object(void **scene, unsigned int obj_id);

extern void *clone_scene(void **scene);

#endif //CWS_RENGINES__ENGINE_H_
