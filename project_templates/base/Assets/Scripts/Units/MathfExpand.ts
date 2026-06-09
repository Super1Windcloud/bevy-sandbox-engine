class MathfExpand
{
	/**
	 * 该向量应用一个旋转
	 * @param q 四元数
	 * @param v 三维向量
	 * @returns 旋转后的三维向量
	 */
	public static rotateVectorByQuaternion(q: Quaternion, v: Vector3): Vector3
	{
		// 将向量v转换为四元数的形式，w分量为0。
		const vectorAsQuaternion = new Quaternion(v.x, v.y, v.z, 0);
	
		// 使用四元数乘法来旋转向量：q * v * q^-1
		// q^-1 是 q 的逆四元数。
		// 在四元数乘法中，两个四元数相乘的结果是一个新的四元数，它表示两个旋转的组合。
		// 这里的旋转是：先应用q表示的旋转，然后应用q的逆旋转，这样可以得到只旋转向量的效果。
		const result = q.Mul(vectorAsQuaternion).Mul(q.inverse);
	
		// 返回旋转后的向量，只需x、y、z分量。
		return new Vector3(result.x, result.y, result.z);
	}
}