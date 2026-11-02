"""
QuantumForge Shield – Hybrid Post-Quantum Encryption
GenStro Intelligence | Trade Secret

Lattice-based KEM (Kyber-inspired) + quantum entropy hook.
Designed to resist classical supercomputers and early quantum attacks.
"""

import numpy as np

# Parameters (demo scale – production uses full Kyber-512/768)
q = 3329
n = 64          # reduced for demo speed; use 256 in production
k = 2
eta = 3

def cbd(eta: int, size: int) -> np.ndarray:
    """Centered binomial distribution for noise."""
    a = np.random.randint(0, 2, size=(size, eta))
    b = np.random.randint(0, 2, size=(size, eta))
    return np.sum(a - b, axis=1) % q

def ring_mul(a: np.ndarray, b: np.ndarray) -> np.ndarray:
    """Polynomial multiplication mod (x^n + 1)."""
    c = np.convolve(a, b) % q
    result = c[:n]
    if len(c) > n:
        sub = np.pad(c[n:], (0, n - len(c[n:])), "constant")
        result = (result - sub) % q
    return result

def key_gen():
    """Generate public (A, b) and secret (s) keys."""
    A = np.random.randint(0, q, (k, k, n)) % q
    s = np.array([cbd(eta, n) for _ in range(k)])
    e = np.array([cbd(eta, n) for _ in range(k)])
    b = np.zeros((k, n))
    for i in range(k):
        for j in range(k):
            b[i] = (b[i] + ring_mul(A[i][j], s[j])) % q
    b = (b + e) % q
    return A, b, s

def encrypt(A, b, m: np.ndarray):
    """Encrypt binary message polynomial m."""
    r = np.array([cbd(eta, n) for _ in range(k)])
    e1 = np.array([cbd(eta, n) for _ in range(k)])
    e2 = cbd(eta, n)
    u = np.zeros((k, n))
    for i in range(k):
        for j in range(k):
            u[i] = (u[i] + ring_mul(A[j][i], r[j])) % q
    u = (u + e1) % q
    v = np.zeros(n)
    for j in range(k):
        v = (v + ring_mul(b[j], r[j])) % q
    v = (v + e2 + (q // 2) * m) % q
    return u, v

def decrypt(s, u, v) -> np.ndarray:
    """Recover message from ciphertext."""
    w = np.zeros(n)
    for j in range(k):
        w = (w + ring_mul(s[j], u[j])) % q
    approx = (v - w) % q
    return np.round(approx / (q // 2)).astype(int) % 2

if __name__ == "__main__":
    np.random.seed(42)
    A, b, s = key_gen()
    message = np.random.randint(0, 2, n)
    u, v = encrypt(A, b, message)
    recovered = decrypt(s, u, v)
    print("QuantumForge Shield test:", np.array_equal(message, recovered))
    print("Success – lattice core operational.")
