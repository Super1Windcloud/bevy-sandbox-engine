class PlayerController extends Component
{
	// ---- Base Attributes ----
	@EditorComponentSettings.DecorateName("角色重力")
	public Gravity: number = -9.81;

	// ---- Component Attributes ----
	@EditorComponentSettings.DecorateName("角色动画控制器")
	public Animator: Animator;
	@EditorComponentSettings.DecorateName("角色摄像机")
	public Cam: Camera;
	@EditorComponentSettings.DecorateName("角色控制器")
	private _characterController: CharacterController;
	@EditorComponentSettings.DecorateName("摄像机控制器")
	private _camController: CameraController;

	// ---- Move And Rotate Attributes ----
	@EditorComponentSettings.DecorateName("角色移动速度")
	public MoveSpeed: number = 2;
	@EditorComponentSettings.DecorateName("角色跳跃速度")
	public JumpSpeed: number = 5;
	@EditorComponentSettings.DecorateName("角色跳跃高度")
	public JumpHeight: number = 1;

	@EditorComponentSettings.DecorateName("角色地面偏移")
	public GroundedOffset: number = -1;
	@EditorComponentSettings.DecorateName("角色转身速度")
	public RotateSpeed: number = 10;
	@EditorComponentSettings.DecorateName("角色转身平滑时间")
	public RotationSmoothTime: number = 0.1;
	@EditorComponentSettings.DecorateName("角色转身平滑速度")
	private _moveDirection: Vector3 = Vector3.zero;
	@EditorComponentSettings.DecorateName("角色转身平滑速度")
	private _moveHorizontal: number = 0;
	@EditorComponentSettings.DecorateName("角色转身平滑速度")
	private _moveVertical: number = 0;
	@EditorComponentSettings.DecorateName("角色转身平滑速度")
	private _groundDirection: Vector3 = Vector3.zero;

	// ---- State ----
	private _isJumping: boolean = false;					// 角色是否正在跳跃
	private _isGrounded: boolean = false;				// 角色是否在地面上

	// ---- Grounded Check ----
	@EditorComponentSettings.DecorateName("检测半径")
	public CheckRadius: number = 0.1;
	@EditorComponentSettings.DecorateName("检测距离")
	public CheckDistance: number = 0.2;
	@EditorComponentSettings.DecorateName("检测层级")
	@EditorComponentSettings.Tooltip("二进制数字")
	public CheckMask: number;

	// ---- Debug ----
	@EditorComponentSettings.DecorateName("显示检测球体")
	public ShowGroundEetection: boolean = false;

	// ---- Engine Function ----
	public OnStart(): void
	{
		this._characterController = this.gameObject.GetComponent<CharacterController>(CharacterController);
		this._camController = this.Cam.gameObject.GetComponent<CameraController>(CameraController);

		if (this._characterController == null || this.Animator == null)
		{
			Debug.Warning("PlayerController.OnStart: characterController or Animator is null.");
		}
	}

	public OnUpdate(): void
	{
		this.JumpAndGravity();
		this.GroundedCheck();
		this.Move();
	}

	// 检查角色是否接触地面
	private GroundedCheck(): void
	{
		// 计算检测起点，稍微在角色位置上方
		let origin: Vector3 = this.transform.position.Add(Vector3.up.Mul(this.GroundedOffset));

		// 向下发射一个球形碰撞器，用于检测地面
		const ray: Ray = new Ray(origin, Vector3.down);
		let hit: RaycastHit = Physics.SphereCast(ray, this.CheckRadius, this.CheckDistance, this.CheckMask);
		// 如果检测到了碰撞体，则认为角色接触地面
		this._isGrounded = hit != undefined;

		// 如果有动画控制器，则更新动画控制器中的接地状态
		if (this.Animator)
		{
			this.Animator.SetBool("InGround", this._isGrounded);
		}

		// 如果需要显示地面检测，绘制球形体来表示检测范围
		if (this.ShowGroundEetection)
		{
			Gizmos.DrawSphere(ray.origin, this.CheckRadius, true);
			Gizmos.DrawWireSphere(ray.origin.Add(Vector3.down.Mul(this.CheckDistance)), this.CheckRadius, true);
		}
	}

	// 处理跳跃和重力
	private JumpAndGravity(): void
	{
		// 如果角色接触地面
		if (this._isGrounded)
		{
			// 如果角色尝试跳跃
			if (this._isJumping)
			{
				// 计算跳跃速度
				this._groundDirection.y = Mathf.Sqrt(this.JumpHeight * -2 * this.Gravity);

				// 如果有动画控制器，则设置跳跃动画，并设置跳跃状态为true
				if (this.Animator)
				{
					this.Animator.SetTrigger("Jump");
					this.Animator.SetBool("InJump", true);
				}

				// 重置跳跃状态
				this._isJumping = false;
			}
			else if (this._groundDirection.y < 0)
			{
				// 如果不跳跃且下降速度小于0，则设置为微小的下降速度，表示接触地面
				this._groundDirection.y = -2;

				// 如果之前是跳跃状态，更新动画控制器并将跳跃状态设置为false
				if (this.Animator && this.Animator.GetBool("InJump") == true)
					this.Animator.SetBool("InJump", false);
			}
		}
		else
		{
			// 如果角色不接触地面，则应用重力
			this._groundDirection.y += this.Gravity * this.JumpSpeed * Time.deltaTime;
		}

		// 使用角色控制器移动角色
		this._characterController.Move(this._groundDirection.Mul(Time.deltaTime));
	}

	// 处理角色移动
	private Move(): void
	{
		// 根据输入方向创建移动向量
		const moveDir: Vector3 = new Vector3(this._moveHorizontal, 0, this._moveVertical);

		// 如果移动向量非零，则归一化
		this._moveDirection = moveDir.magnitude != 0 ? moveDir.normalized : Vector3.zero;

		// 如果移动方向的大小超过阈值
		if (this._moveDirection.magnitude >= 0.1)
		{
			// 计算目标旋转角度
			const targetAngle: number = Mathf.ATan2(this._moveDirection.x, this._moveDirection.z) * Mathf.radToAngle + this.Cam.transform.eulerAngles.y;

			// 计算当前和目标旋转的四元数
			let currentQuaternion = this.transform.rotation;
			let targetQuaternion = Quaternion.FromEulerXYZ(0, targetAngle, 0);

			// 使用Slerp进行旋转插值
			let t = Time.deltaTime * this.RotateSpeed;
			let interpolatedQuaternion = Quaternion.Slerp(currentQuaternion, targetQuaternion, t);

			// 应用插值后的旋转
			this.transform.rotation = interpolatedQuaternion;

			// 使用插值后的旋转计算移动方向
			const moveDir: Vector3 = MathfExpand.rotateVectorByQuaternion(interpolatedQuaternion, Vector3.forward);

			// 使用角色控制器以一定速度移动角色
			this._characterController.Move(moveDir.normalized.Mul(this.MoveSpeed * Time.deltaTime));

			// 如果有动画控制器，则更新移动速度参数则更新移动速度参数
			if (this.Animator)
				this.Animator.SetFloat("Move", this._moveDirection.magnitude);
		}
		else
		{
			// 如果有动画控制器，且移动方向的大小不足阈值，则更新动画控制器参数为0
			if (this.Animator)
				this.Animator.SetFloat("Move", 0);
		}
	}

	// 公共方法，用于设置角色的移动方向
	public CharacterMove(vec: Vector2): void
	{
		this._moveHorizontal = vec.x;
		this._moveVertical = vec.y;
	}

	// 公共方法，用于设置角色的视角控制
	public CharacterView(vec: Vector2): void
	{
		this._camController.CameraView(vec);
	}

	// 公共方法，用于使角色跳跃
	public CharacterJump(): void
	{
		// 如果角色接触地面，则设置跳跃状态为true
		if (this._isGrounded)
			this._isJumping = true;
	}

}