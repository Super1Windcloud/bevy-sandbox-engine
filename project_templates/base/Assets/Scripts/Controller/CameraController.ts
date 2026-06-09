class CameraController extends Component
{
	@EditorComponentSettings.DecorateName("摄像机跟随目标")
	public CameraFollow: GameObject;
	@EditorComponentSettings.DecorateName("摄像机最大距离")
	public MaxDistance: number = 5;
	@EditorComponentSettings.DecorateName("摄像机最小距离")
	public MinDistance: number = 1;
	@EditorComponentSettings.DecorateName("摄像机最大旋转角度")
	public MaxRotateAngle: number = 60;
	@EditorComponentSettings.DecorateName("摄像机最小旋转角度")
	public MinRotateAngle: number = -60;
	@EditorComponentSettings.DecorateName("摄像机旋转灵敏度")
	public RotateSensitivity: number = 30;
	@EditorComponentSettings.DecorateName("摄像机碰撞层级")
	public HitLayerMask: number;
	@EditorComponentSettings.DecorateName("摄像机碰撞偏移")
	public Offset: number = 0.1;
	@EditorComponentSettings.DecorateName("平滑速度")
	public SmoothSpeed: number = 0.1;

	private _viewHorizontal: number = 0;    // 相机横向输入轴
	private _viewVertical: number = 0;      // 相机纵向输入轴
	private _yaw: number = 0;				// 水平旋转角度
	private _pitch: number = 0;				// 垂直旋转角度
	private _outDistance: number = 2;		// 摄像机距离目标的距离

	public OnLateUpdate(): void
	{
		this.CameraMove();
	}

	// 定义一个私有的方法用于处理相机的移动
	private CameraMove(): void
	{
		// 根据水平视角输入调整偏航角（水平旋转）
		this._yaw += this._viewHorizontal;

		// 根据垂直视角输入调整俯仰角（垂直旋转），并确保它在最小旋转角度和最大旋转角度之间
		this._pitch -= this._viewVertical;
		this._pitch = Mathf.Clamp(this._pitch, this.MinRotateAngle, this.MaxRotateAngle);

		// 定义一个用于射线检测结果的变量
		let hit: RaycastHit = new RaycastHit();

		// 创建一条从相机跟随目标位置到当前相机位置的射线
		const ray: Ray = new Ray(this.CameraFollow.transform.position, this.transform.position.Sub(this.CameraFollow.transform.position));
		
		// 进行射线检测，检测到的物体必须在HitLayerMask层内，并且射线长度不超过MaxDistance
		hit = Physics.Raycast(ray, this.MaxDistance, this.HitLayerMask);
		if (hit)
		{
			// 如果射线检测到物体，计算距离并减去一个偏移量
			this._outDistance = Vector3.Distance(this.CameraFollow.transform.position, hit.point) - this.Offset;
			
			// 确保计算出的距离在区间内
			if (this._outDistance < this.MinDistance)
				this._outDistance = this.MinDistance;
			if (this._outDistance > this.MaxDistance)
				this._outDistance = this.MaxDistance;
		}
		else
		{
			// 如果射线没有检测到物体，则设置距离为最大距离
			this._outDistance = this.MaxDistance;
		}

		// 根据当前的俯仰角和偏航角创建一个旋转
		const rotation: Quaternion = Quaternion.FromEulerXYZ(this._pitch, this._yaw, 0);

		// 计算相机应该在的目标位置
		const targetPosition: Vector3 =
			this.CameraFollow.transform.position.Add(
				MathfExpand.rotateVectorByQuaternion(rotation, Vector3.back)
					.Mul(this._outDistance)
			);

		// 平滑地移动相机到目标位置
		this.transform.position = Vector3.Lerp(this.transform.position, targetPosition, this.SmoothSpeed);
		
		// 让相机朝向跟随目标
		this.transform.LookAt(this.CameraFollow.transform.position);
	}

	// 公共方法，用于接收从外部传入的视角控制向量，通常由玩家输入控制
	public CameraView(vec: Vector2): void
	{
		// 根据玩家输入调整水平和垂直视角，乘以旋转灵敏度和时间差，以保证平滑的旋转效果
		this._viewHorizontal = vec.x * this.RotateSensitivity * Time.deltaTime;
		this._viewVertical = vec.y * this.RotateSensitivity * Time.deltaTime;
	}

}