class JoyStick extends Component
{
	@EditorComponentSettings.DecorateName("是否启用摇杆输入")
	public IsJoyStickInputEnable: boolean = true;

	@EditorComponentSettings.DecorateName("控制目标")
	public ControlTarget: PlayerController;

	@EditorComponentSettings.DecorateName("左摇杆")
	public LeftJoyStick: string;
	@EditorComponentSettings.DecorateName("左摇杆盖")
	public LeftJoyStickCap: string;
	@EditorComponentSettings.DecorateName("屏幕移动区域")
	public ScreenMoveArea: string;
	@EditorComponentSettings.DecorateName("屏幕移动死区")
	public ScreenMoveDeadZone: number = 5;
	@EditorComponentSettings.DecorateName("跳跃按钮")
	public JumpButton: string;

	private _uiComponent: UIComponent;			// UI组件
	private _leftJoyStick: Image;				// 左摇杆
	private _leftJoyStickCap: Image;			// 左摇杆盖
	private _jumpButton: Image;					// 跳跃按钮
	private _screenMoveArea: Image;				// 屏幕移动区域

	public OnStart(): void
	{
		this._uiComponent = this.gameObject.GetComponent<UIComponent>(UIComponent);
		const canvas: Canvas = this._uiComponent.canvas;

		this._leftJoyStick = canvas.FindChild<Image>(Image,this.LeftJoyStick);
		this._leftJoyStickCap = this._leftJoyStick.FindChild<Image>(Image,this.LeftJoyStickCap);
		this._jumpButton = canvas.FindChild<Image>(Image,this.JumpButton);
		this._screenMoveArea = canvas.FindChild<Image>(Image,this.ScreenMoveArea);

		this._leftJoyStick.AddEvent(ControlEvent.EventPointerDrag, (control: Control) =>
		{
			this.ControlTarget.CharacterMove(this.PlayerMove(this._leftJoyStick, this._leftJoyStickCap));
		});

		this._leftJoyStick.AddEvent(ControlEvent.EventPointerStationary, (control: Control) =>
		{
			// 获取GUI事件数据
			const myEventData: GUIEventData = this._leftJoyStick.guiEventData;

			this.ControlTarget.CharacterMove(this.PlayerMove(this._leftJoyStick, this._leftJoyStickCap));
		});

		this._leftJoyStick.AddEvent(ControlEvent.EventPointerUp, (control: Control) =>
		{
			// 获取摇杆帽
			const thumb = this._leftJoyStick.FindChild<Image>(Image,this.LeftJoyStickCap);

			// 获取摇杆帽的RectTransform
			const thumbRectTransform = thumb.rectTransform;

			// 归位
			thumbRectTransform.localPosition = Vector2.zero;

			this.ControlTarget.CharacterMove(Vector2.zero);
		});

		this._screenMoveArea.AddEvent(ControlEvent.EventPointerDrag, (control: Control) =>
		{
			// 获取GUI事件数据
			const myEventData: GUIEventData = this._screenMoveArea.guiEventData;

			// 获取拖拽位置
			let dragPosition = myEventData.moveDelta;
			dragPosition.y = -dragPosition.y;

			this.ControlTarget.CharacterView(dragPosition);
		});

		this._screenMoveArea.AddEvent(ControlEvent.EventPointerStationary, (control: Control) =>
		{
			this.ControlTarget.CharacterView(Vector2.zero);
		});

		this._screenMoveArea.AddEvent(ControlEvent.EventPointerUp, (control: Control) =>
		{
			this.ControlTarget.CharacterView(Vector2.zero);
		});

		this._jumpButton.AddEvent(ControlEvent.EventPointerDown, (control: Control) =>
		{
			this.ControlTarget.CharacterJump();
		});
	}

	private PlayerMove(leftJoyStick:Image, leftJoyStickCap: Image): Vector2
	{
		// 获取GUI事件数据
		const myEventData: GUIEventData = leftJoyStick.guiEventData;

		// 获取摇杆帽的RectTransform
		const thumbRectTransform = leftJoyStickCap.rectTransform;

		// 获取摇杆的RectTransform
		const joystickRectTransform = leftJoyStick.rectTransform;

		// 获取拖拽位置
		const dragPosition = myEventData.position;

		// 获取摇杆的中心位置
		const radius = joystickRectTransform.width / 2;

		// 获取摇杆的原始位置
		const oriPos = joystickRectTransform.worldPosition.Add(new Vector2(radius, -radius));

		// 获取摇杆的偏移
		const offset = dragPosition.Sub(oriPos);

		// 摇杆帽的位置
		let thumbPos = new Vector2();

		if (offset.magnitude > radius)
		{
			thumbPos = offset.normalized.Mul(radius);
		}
		else
		{
			thumbPos = offset;
		}

		// 更新摇杆帽的位置
		thumbRectTransform.localPosition = thumbPos;

		const move: Vector2 = new Vector2(offset.x, -offset.y).normalized;

		return move;
	}
}