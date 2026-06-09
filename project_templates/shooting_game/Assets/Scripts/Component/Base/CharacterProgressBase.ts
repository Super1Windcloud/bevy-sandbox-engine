abstract class CharacterProgressBase extends Component
{
	@EditorComponentSettings.DecorateName("目标")
	public Target!: GameObject;				// 目标
	@EditorComponentSettings.DecorateName("摄像机")
	public Camera!: Camera;					// 摄像机
	@EditorComponentSettings.DecorateName("偏移量")
	public Offest: Vector3 = new Vector3();	// 偏移量
	@EditorComponentSettings.DecorateName("透视修正参数")
	public DepthFactor: number = 0.1;		// 透视修正参数
	@EditorComponentSettings.DecorateName("进度条组名字")
	public ProgressGroupName!: string;		// 进度条组名字

	protected _uiComponent!: UIComponent;
	protected _progressGroup!: Image;

	public OnStart(): void
	{
		if (this.Camera == null)
		{
			this.Camera = Camera.mainCamera;
		}
		const uiComponent = this.gameObject.GetComponent<UIComponent>(UIComponent);
		if (uiComponent == null)
		{
			Debug.Warning("CharacterProgressBase: UIComponent is null.");
			return;
		}
		const progressGroup = uiComponent.canvas.FindChild<Image>(Image, this.ProgressGroupName);
		if (progressGroup == null)
		{
			Debug.Warning("CharacterProgressBase: ProgressGroup is null.");
			return;
		}
		this._uiComponent = uiComponent;
		this._progressGroup = progressGroup;

		this.ContinueStart();
	}

	public OnUpdate(): void
	{
		// 在加载场景后防止无法立刻获取到摄像机
		if (this.Camera == null || this.Target == null)
			return;
		if (this._progressGroup == null)
			return;

		this.SyncTarget();

		this.ContinueUpdate();
	}

	public SyncTarget(): void
	{
		let distanceToCamera: number = Vector3.Distance(this.Target.transform.position, this.Camera.transform.position);
		let adjustedOffset: Vector3 = this.Offest.Add(this.Offest.normalized.Mul(distanceToCamera * this.DepthFactor));
		let screenPosition: Vector3 = this.Camera.WorldPointToScreen(this.Target.transform.position.Add(adjustedOffset));
		this._progressGroup.rectTransform.localPosition = new Vector2(screenPosition.x, -screenPosition.y);
	}

	// abstract
	protected abstract ContinueStart(): void;
	protected abstract ContinueUpdate(): void;
}
