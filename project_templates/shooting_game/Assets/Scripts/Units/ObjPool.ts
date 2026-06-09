class ObjPool
{
	private _pooledAmount: number = 0;		// 预制体最大生成数量
	private _willGrow: boolean = true;			// 是否允许超过上限
	private _objectPrefab!: Prefab;		// 预制体
	private _parentObject: Transform | undefined;	// 父物体
	private _poolObjectList: List<GameObject> = new List(GameObject);	// 对象池

	/**
	 * 初始化 Pool
	 * @param willGrow 是否允许超过上限
	 * @param prefabUUID 预制体 UUID
	 * @param pooledAnount 预制体数量
	 * @param parentObject 指定预制体的父物体
	 */
	public Init(prefab: Prefab, pooledAnount: number = 20,
		willGrow: boolean = true, parentObject?: Transform): void
	{
		// 初始化参数
		this._pooledAmount = pooledAnount;
		this._willGrow = willGrow;
		this._parentObject = parentObject;

		// 获取预制体资源
		this._objectPrefab = prefab;

		// 初始化生成
		for (let i = 0; i < this._pooledAmount; i++)
		{
			this.InstancePrefab(this._parentObject);
		}
	}

	/**
	 * 从对象池中获取一个未激活的 OBJ, 如果没有则返回 undefined
	 * @returns 未激活的 OBJ | undefined
	 */
	public GetPooledObject(): GameObject | undefined
	{
		// 检查当前库存中是否有未激活的 OBJ
		for (let i = 0; i < this._poolObjectList.count; i++)
		{
			let obj = this._poolObjectList.get(i);
			if (obj.enable == false)
				return obj;
		}

		// 如果允许超过上限，则创建一个新的 OBJ
		if (this._willGrow)
		{
			return this.InstancePrefab(this._parentObject);
		}

		return undefined;
	}

	/**
	 * 生成一个新的OBJ
	 * @param parentObject 该OBJ的父物体
	 * @returns 生成的OBJ
	 */
	private InstancePrefab(parentObject?: Transform): GameObject
	{
		let obj: GameObject = this._objectPrefab.Instance();
		if (parentObject != undefined)	// 检查是否存在父物体
		{
			obj.transform.parent = parentObject;
			obj.transform.localPosition = Vector3.zero;
			obj.transform.localRotation = Quaternion.identity;
		}

		obj.enable = false;
		this._poolObjectList.Add(obj);

		return obj;
	}
}
