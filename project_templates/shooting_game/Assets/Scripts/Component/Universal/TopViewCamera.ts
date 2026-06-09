class TopViewCamera extends Component
{
	@EditorComponentSettings.DecorateName("相机距离目标的距离")
	public CameraDistance: number = 15;
	@EditorComponentSettings.DecorateName("相机与目标的角度偏移")
	public AngleOffsetX: number = 20;
	@EditorComponentSettings.DecorateName("相机的角度")
	public CameraRotateX: number = 65;
	@EditorComponentSettings.DecorateName("相机看向的目标")
	public LookAtTarget: GameObject | null = null;
	@EditorComponentSettings.DecorateName("位置插值速度")
	public positionLerpSpeed: number = 0.1;
	@EditorComponentSettings.DecorateName("旋转插值速度")
	public rotationLerpSpeed: number = 0.1;

	private _offset: Vector3 = Vector3.zero;	// 相机与目标的偏移

	public OnLateUpdate(): void
	{
		if (this.LookAtTarget == null)
			return;

		// 计算相机与目标的偏移
		let desiredOffset: Vector3 = Quaternion.FromEulerXYZ(-this.AngleOffsetX, 0, 0).RotateVector(new Vector3(0, this.CameraDistance, 0));

		// 使用Lerp插值计算相机新的位置
		this._offset = Vector3.Lerp(this._offset, desiredOffset, this.positionLerpSpeed);

		// 设置相机位置
		let desiredPosition: Vector3 = this.LookAtTarget.transform.position.Add(this._offset);
		this.gameObject.transform.position =Vector3.Lerp(this.gameObject.transform.position, desiredPosition, this.positionLerpSpeed);

		// 设置相机旋转
		let desiredRotation: Quaternion = Quaternion.FromEulerXYZ(this.CameraRotateX, 0, 0);
    	this.gameObject.transform.rotation = Quaternion.Lerp(this.gameObject.transform.rotation, desiredRotation, this.rotationLerpSpeed);
	}
}
