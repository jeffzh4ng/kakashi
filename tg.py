from tinygrad.tensor import Tensor

a = Tensor([1, 2])
b = Tensor([3, 4])
res = a.dot(b).numpy()
print(res)
