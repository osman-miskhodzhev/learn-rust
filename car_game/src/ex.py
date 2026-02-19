import math

# Параметры авто
L = 2.5  # колесная база в метрах
dt = 0.1 # шаг времени (секунды)

# Состояние: [x, y, theta, speed]
state = [0.0, 0.0, 0.0, 0.0]

def update(state, throttle, steering_angle):
    """
    throttle: сила тяги (может быть -1..1, но лучше в м/с^2)
    steering_angle: угол поворота колес (в радианах, например, -0.5 .. 0.5)
    """
    x, y, theta, speed = state

    # Простейшая динамика скорости (разгон/трение)
    acceleration = throttle * 5.0  # множитель для чувствительности
    friction = 0.1
    speed = speed + (acceleration - friction * speed) * dt
    # Ограничим скорость (опционально)
    # speed = max(0, min(30, speed))

    # Кинематика поворота
    if abs(speed) < 0.01:  # если стоим, угол не меняем
        theta = theta
    else:
        theta = theta + (speed / L) * math.tan(steering_angle) * dt

    # Новые координаты
    x = x + speed * math.cos(theta) * dt
    y = y + speed * math.sin(theta) * dt

    return [x, y, theta, speed]

# Пример использования
state = update(state, throttle=1.0, steering_angle=0.2)
print(f"X: {state[0]:.2f}, Y: {state[1]:.2f}, Угол: {state[2]:.2f}, Скорость: {state[3]:.2f}")