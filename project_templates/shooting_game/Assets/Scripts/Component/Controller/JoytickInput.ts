class JoytickInput extends Component
{
	@EditorComponentSettings.DecorateName("是否启用摇杆输入")
	public IsJoystickInputEnable: boolean = true;

	@EditorComponentSettings.DecorateName("左摇杆名")
	public LeftJoystickName: string = "LeftJoystick";
	@EditorComponentSettings.DecorateName("左摇杆帽名")
	public LeftJoyStickCapName: string = "LeftJoystickCap";
	@EditorComponentSettings.DecorateName("右摇杆名")
	public RightJoystickName: string = "RightJoystick";
	@EditorComponentSettings.DecorateName("右摇杆帽名")
	public RightJoyStickCapName: string = "RightJoystickCap";

	private _uiComponent: UIComponent | null = null;							// UI组件
	private _leftJoystick: Image | null = null;								// 左摇杆
	private _rightJoystick: Image | null = null;								// 右摇杆

	public OnStart(): void
	{
		this._uiComponent = this.gameObject.GetComponent<UIComponent>(UIComponent);
		if (this._uiComponent == null)
		{
			Debug.Warning("JoytickInput: UIComponent is null.");
			return;
		}
		let canvas: Canvas = this._uiComponent.canvas;

		this._leftJoystick = canvas.FindChild<Image>(Image, this.LeftJoystickName);
		this._rightJoystick = canvas.FindChild<Image>(Image, this.RightJoystickName);
		if (this._leftJoystick == null || this._rightJoystick == null)
		{
			Debug.Warning("JoytickInput: joystick control is null.");
			return;
		}

		this._leftJoystick.AddEvent(ControlEvent.EventPointerDrag, (_control: Control) =>
		{
			this.LeftJoyStickInputEnable();
		});

		this._leftJoystick.AddEvent(ControlEvent.EventPointerStationary, (_control: Control) =>
		{
			this.LeftJoyStickInputEnable();
		});

		this._rightJoystick.AddEvent(ControlEvent.EventPointerDrag, (_control: Control) =>
		{
			this.RightJoyStickInputEnable();
		});

		this._rightJoystick.AddEvent(ControlEvent.EventPointerStationary, (_control: Control) =>
		{
			this.RightJoyStickInputEnable();
		});

		this._leftJoystick.AddEvent(ControlEvent.EventPointerUp, (_control: Control) =>
		{
			const leftJoystick = this._leftJoystick;
			if (leftJoystick == null)
				return;

			// 获取摇杆帽
			const thumb = leftJoystick.FindChild<Image>(Image, this.LeftJoyStickCapName);
			if (thumb == null)
				return;

			// 获取摇杆帽的RectTransform
			const thumbRectTransform = thumb.rectTransform;

			// 归位
			thumbRectTransform.localPosition = Vector2.zero;

			GlobalEvent.Instance.Publish(EventName.Player_Controller_Move, Vector3.zero);	// 发布移动事件
		});

		this._rightJoystick.AddEvent(ControlEvent.EventPointerUp, (_control: Control) =>
		{
			const rightJoystick = this._rightJoystick;
			if (rightJoystick == null)
				return;

			// 获取摇杆帽
			const thumb = rightJoystick.FindChild<Image>(Image, this.RightJoyStickCapName);
			if (thumb == null)
				return;

			// 获取摇杆帽的RectTransform
			const thumbRectTransform = thumb.rectTransform;

			// 归位
			thumbRectTransform.localPosition = Vector2.zero;

			GlobalEvent.Instance.Publish(EventName.Player_Controller_Rotate, Vector3.zero);	// 发布移动事件
		})
	}

	private LeftJoyStickInputEnable(): void
	{
		if (this._leftJoystick == null)
			return;

		// 获取GUI事件数据
		const myEventData: GUIEventData = this._leftJoystick.guiEventData;

		// 获取摇杆帽
		const thumb = this._leftJoystick.FindChild<Image>(Image, this.LeftJoyStickCapName);
		if (thumb == null)
			return;

		// 获取摇杆帽的RectTransform
		const thumbRectTransform = thumb.rectTransform;

		// 获取摇杆的RectTransform
		const joystickRectTransform = this._leftJoystick.rectTransform;

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

		const move: Vector3 = new Vector3(offset.x, 0, -offset.y).normalized;

		GlobalEvent.Instance.Publish(EventName.Player_Controller_Move, move);	// 发布移动事件
	}

	private RightJoyStickInputEnable(): void
	{
		if (this._rightJoystick == null)
			return;

		// 获取GUI事件数据
		const myEventData: GUIEventData = this._rightJoystick.guiEventData;

		// 获取摇杆帽
		const thumb = this._rightJoystick.FindChild<Image>(Image, this.RightJoyStickCapName);
		if (thumb == null)
			return;

		// 获取摇杆帽的RectTransform
		const thumbRectTransform = thumb.rectTransform;

		// 获取摇杆的RectTransform
		const joystickRectTransform = this._rightJoystick.rectTransform;

		// 获取拖拽位置
		const dragPosition = myEventData.position;

		// 获取摇杆的中心位置
		const radius = joystickRectTransform.width / 2;

		// 获取摇杆的原始位置
		const oriPos = joystickRectTransform.worldPosition.Add(new Vector2(-radius, -radius));

		// 获取摇杆的偏移
		const offset = dragPosition.Sub(oriPos);

		// 摇杆帽的位置
		let thumbPos = new Vector2();

		// 开火标记
		let weaponFire: boolean = false;

		// 检查开火条件
		if (offset.x > radius || offset.y > radius || offset.x < -radius || offset.y < -radius)
		{
			weaponFire = true;
		}

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

		if (weaponFire)
			GlobalEvent.Instance.Publish(EventName.Weapon_Rifle_Shoot, true);	// 发布开火事件

		const rotate: Vector3 = new Vector3(offset.x, 0, -offset.y).normalized;

		GlobalEvent.Instance.Publish(EventName.Player_Controller_Rotate, rotate);	// 发布移动事件
	}
}
