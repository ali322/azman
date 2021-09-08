-- Add migration script here
DROP TABLE IF EXISTS `users`;

CREATE TABLE IF NOT EXISTS `domains`(
  `id` varchar(100) NOT NULL,
  `name` VARCHAR(100) NOT NULL,
  `description` TEXT,
  `default_role_id` INTEGER,
  `admin_role_id` INTEGER,
  `is_deleted` int(1) NOT NULL DEFAULT '0',
  `created_at` TIMESTAMP NOT NULL DEFAULT NOW(),
  `updated_at` TIMESTAMP NOT NULL DEFAULT NOW()
);

DROP TABLE IF EXISTS `users`;

CREATE TABLE `users` (
  `id` varchar(100) NOT NULL,
  `username` varchar(50) NOT NULL,
  `password` varchar(100) NOT NULL,
  `email` varchar(200) DEFAULT NULL,
  `last_logined_at` datetime NOT NULL,
  `created_at` datetime NOT NULL,
  `avatar` TEXT,
  `memo` TEXT,
  `sys_role` VARCHAR(50) NOT NULL,
  `is_actived` int(1) NOT NULL DEFAULT '1',
  PRIMARY KEY (`id`)
);

CREATE TABLE IF NOT EXISTS `roles`(
  `id` INT NOT NULL AUTO_INCREMENT,
  `name` VARCHAR(100) NOT NULL,
  `description` TEXT,
  `value` VARCHAR(200) NOT NULL,
  `level` INTEGER NOT NULL,
  `is_deleted` int(1) NOT NULL DEFAULT '0',
  `domain_id` VARCHAR(100) NOT NULL REFERENCES `domains`(`id`),
  `created_at` TIMESTAMP NOT NULL DEFAULT NOW(),
  `updated_at` TIMESTAMP NOT NULL DEFAULT NOW(),
  `created_by` VARCHAR(100) NOT NULL REFERENCES `users`(`id`),
  `updated_by` VARCHAR(100) NOT NULL REFERENCES `users`(`id`),
  PRIMARY KEY (`id`)
);

CREATE TABLE IF NOT EXISTS `user_has_roles`(
  `user_id` VARCHAR(100) REFERENCES `users`(`id`),
  `role_id` INTEGER REFERENCES `roles`(`id`),
  `expire` TIMESTAMP NOT NULL,
  `created_at` TIMESTAMP NOT NULL DEFAULT NOW(),
  PRIMARY KEY(`user_id`, `role_id`)
);

CREATE TABLE IF NOT EXISTS `perms`(
  `id` INT NOT NULL AUTO_INCREMENT,
  `name` VARCHAR(100) NOT NULL,
  `description` TEXT,
  `value` TEXT,
  `is_deleted` int(1) NOT NULL DEFAULT '0',
  `domain_id` VARCHAR(100) NOT NULL REFERENCES `domains`(`id`),
  `created_at` TIMESTAMP NOT NULL DEFAULT NOW(),
  `updated_at` TIMESTAMP NOT NULL DEFAULT NOW(),
  `created_by` VARCHAR(100) NOT NULL REFERENCES `users`(`id`),
  `updated_by` VARCHAR(100) NOT NULL REFERENCES `users`(`id`),
  PRIMARY KEY (`id`)
);

CREATE TABLE IF NOT EXISTS `role_has_perms`(
  `perm_id` INTEGER REFERENCES `perms`(`id`),
  `role_id` INTEGER REFERENCES `roles`(`id`),
  `created_at` TIMESTAMP NOT NULL DEFAULT NOW(),
  PRIMARY KEY(`perm_id`, `role_id`)
);

CREATE TABLE IF NOT EXISTS `orgs` (
  `id` varchar(100) NOT NULL,
  `name` VARCHAR(100) NOT NULL,
  `description` TEXT,
  `is_deleted` int(1) NOT NULL DEFAULT '0',
  `domain_id` VARCHAR(100) NOT NULL REFERENCES `domains`(`id`),
  `created_at` TIMESTAMP NOT NULL DEFAULT NOW(),
  `updated_at` TIMESTAMP NOT NULL DEFAULT NOW(),
  `created_by` VARCHAR(100) NOT NULL REFERENCES `users`(`id`),
  `updated_by` VARCHAR(100) NOT NULL REFERENCES `users`(`id`),
  PRIMARY KEY (`id`)
);

CREATE TABLE IF NOT EXISTS `user_has_orgs`(
  `user_id` VARCHAR(100) REFERENCES `users`(`id`),
  `org_id` VARCHAR(100) REFERENCES `orgs`(`id`),
  `expire` TIMESTAMP NOT NULL,
  `created_at` TIMESTAMP NOT NULL DEFAULT NOW(),
  PRIMARY KEY(`user_id`, `org_id`)
);

INSERT INTO
  `users`(
    `id`,
    `username`,
    `password`,
    `email`,
    `avatar`,
    `memo`,
    `sys_role`,
    `is_actived`,
    `last_logined_at`,
    `created_at`
  )
VALUES
  (
    '0c5b2b97-aefe-4110-80b5-fea91359f5b1',
    'admin',
    '$2b$04$BUPiWXysNDZw3ky8rQMyg.LsKyL80vGWgbwWzSUBLlfOFgqHz8jKq',
    NULL,
    NULL,
    NULL,
    'admin',
    TRUE,
    '2021-07-27 10:36:43.929291',
    '2021-07-27 02:36:44.028063'
  );