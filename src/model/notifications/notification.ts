import { writable, derived, type Readable, type Writable } from "svelte/store"

function createNotificationStore () {
  const _notifications: Writable<Array<Notification>> = writable([]);

  function send(message: string, type = "default", timeout: number) {
    _notifications.update(state => {
      return [...state, { id: id(), type, message, timeout }]
    })
  }

  const notifications: Readable<Array<Notification>> = derived(_notifications, ($_notifications, set) => {
    set($_notifications)
    if ($_notifications.length > 0) {
      const timer = setTimeout(() => {
        _notifications.update(state => {
          state.shift()
          return state
        })
      }, $_notifications[0].timeout)
      return () => {
        clearTimeout(timer)
      }
    }
  })
  const { subscribe } = notifications

  return {
    subscribe,
    send,
    default: (msg: string, timeout: number) => send(msg, "default", timeout),
    red: (msg: string, timeout: number) => send(msg, "danger", timeout),
    warn: (msg: string, timeout: number) => send(msg, "warning", timeout),
    info: (msg: string, timeout: number) => send(msg, "info", timeout),
    success: (msg: string, timeout: number) => send(msg, "success", timeout),
  }
}

function id() {
  return Math.random();
};

export const notifications = createNotificationStore();

export type Notification = {
  id: number,
  type: string,
  message: string,
  timeout: number,
}